use plamo_http_headers::PlamoHttpHeaders;
use plamo_http_method::PlamoHttpMethod;
use plamo_http_queries::PlamoHttpQueries;
use plamo_result::PlamoResult;
use std::collections::BTreeMap;
use std::ffi::{CStr, CString};
use std::os::raw::{c_char, c_uchar};
use std::ptr;

#[repr(C)]
pub struct PlamoRequest {
    method: PlamoHttpMethod,
    scheme: *mut c_char,
    path: *mut c_char,
    query: *mut BTreeMap<CString, PlamoHttpQueries>,
    version: *mut c_char,
    header: *mut BTreeMap<CString, PlamoHttpHeaders>,
    body: *mut Vec<c_uchar>,
}

#[no_mangle]
pub extern fn plamo_request_new(
    method: PlamoHttpMethod,
    scheme: *mut c_char,
    path: *mut c_char,
    version: *mut c_char,
    plamo_request: &mut *mut PlamoRequest
) -> PlamoResult {
    *plamo_request = Box::into_raw(Box::new(PlamoRequest {
        scheme: scheme,
        method: method,
        path: path,
        query: Box::into_raw(Box::new(BTreeMap::new())),
        version: version,
        header: Box::into_raw(Box::new(BTreeMap::new())),
        body: Box::into_raw(Box::new(Vec::new())),
    }));
    PlamoResult::Ok
}

#[no_mangle]
pub extern fn plamo_request_destroy(plamo_request: &mut *mut PlamoRequest) {
    if !plamo_request.is_null() {
        unsafe {
            let plamo_request = Box::from_raw(*plamo_request);
            Box::from_raw(plamo_request.query);
            Box::from_raw(plamo_request.header);
            Box::from_raw(plamo_request.body);
        }
        *plamo_request = ptr::null_mut();
    }
}

#[no_mangle]
pub extern fn plamo_request_header_find(plamo_request: *const PlamoRequest, key: *const c_char, plamo_http_headers: &mut *const PlamoHttpHeaders) -> PlamoResult {
    unsafe {
        match (*(*plamo_request).header).get(CStr::from_ptr(key)) {
            Some(headers) => {
                *plamo_http_headers = headers;
                PlamoResult::Ok
            },
            None => PlamoResult::NotFound,
        }
    }
}

#[no_mangle]
pub extern fn plamo_request_query_find(plamo_request: *const PlamoRequest, key: *const c_char, plamo_http_queries: &mut *const PlamoHttpQueries) -> PlamoResult {
    unsafe {
        match (*(*plamo_request).query).get(CStr::from_ptr(key)) {
            Some(queries) => {
                *plamo_http_queries = queries;
                PlamoResult::Ok
            },
            None => PlamoResult::NotFound,
        }
    }
}
