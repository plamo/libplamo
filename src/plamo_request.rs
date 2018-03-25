use plamo_http_header::{PlamoHttpHeader, plamo_http_header_new};
use plamo_http_method::PlamoHttpMethod;
use plamo_http_query::{PlamoHttpQuery, plamo_http_query_new};
use plamo_result::PlamoResult;
use std::os::raw::{c_char, c_uchar};
use std::ptr;

#[repr(C)]
pub struct PlamoRequest {
    method: PlamoHttpMethod,
    scheme: *mut c_char,
    path: *mut c_char,
    query: *mut PlamoHttpQuery,
    version: *mut c_char,
    header: *mut PlamoHttpHeader,
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
    let mut header = ptr::null_mut();
    plamo_http_header_new(&mut header);
    let mut query = ptr::null_mut();
    plamo_http_query_new(&mut query);
    *plamo_request = Box::into_raw(Box::new(PlamoRequest {
        scheme: scheme,
        method: method,
        path: path,
        query: query,
        version: version,
        header: header,
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
