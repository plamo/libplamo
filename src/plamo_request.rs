use plamo_http_headers::PlamoHttpHeaders;
use plamo_http_method::PlamoHttpMethod;
use plamo_http_queries::PlamoHttpQueries;
use plamo_result::PlamoResult;
use std::collections::BTreeMap;
use std::ffi::CStr;
use std::os::raw::{c_char, c_uchar};
use std::ptr;

#[repr(C)]
pub struct PlamoRequest {
    method: PlamoHttpMethod,
    scheme: *const c_char,
    path: *const c_char,
    query: *const BTreeMap<String, PlamoHttpQueries>,
    version: *const c_char,
    header: *const BTreeMap<String, PlamoHttpHeaders>,
    body: *const Vec<c_uchar>,
}

#[no_mangle]
pub extern fn plamo_request_new(
    method: PlamoHttpMethod,
    scheme: *const c_char,
    path: *const c_char,
    version: *const c_char,
    plamo_request: &mut *const PlamoRequest
) -> PlamoResult {
    *plamo_request = Box::into_raw(Box::new(PlamoRequest {
        scheme: scheme,
        method: method,
        path: path,
        query: &BTreeMap::new(),
        version: version,
        header: &BTreeMap::new(),
        body: &Vec::new(),
    }));
    PlamoResult::Ok
}

#[no_mangle]
pub extern fn plamo_request_header_find(plamo_request: *const PlamoRequest, key: *const c_char, plamo_http_headers: &mut *const PlamoHttpHeaders) -> PlamoResult {
    unsafe {
        match CStr::from_ptr(key).to_str() {
            Ok(key) => {
                match (*(*plamo_request).header).get(key) {
                    Some(ref headers) => {
                        *plamo_http_headers = ptr::read(headers) as *const _;
                        PlamoResult::Ok
                    },
                    None => PlamoResult::NotFound,
                }
            },
            Err(_) => PlamoResult::InvalidString,
        }
    }
}

#[no_mangle]
pub extern fn plamo_request_query_find(plamo_request: *const PlamoRequest, key: *const c_char, plamo_http_queries: &mut *const PlamoHttpQueries) -> PlamoResult {
    unsafe {
        match CStr::from_ptr(key).to_str() {
            Ok(key) => {
                match (*(*plamo_request).query).get(key) {
                    Some(ref queries) => {
                        *plamo_http_queries = ptr::read(queries) as *const _;
                        PlamoResult::Ok
                    },
                    None => PlamoResult::NotFound,
                }
            },
            Err(_) => PlamoResult::InvalidString,
        }
    }
}
