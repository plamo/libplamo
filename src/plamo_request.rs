use plamo_http_headers::PlamoHttpHeaders;
use plamo_http_method::PlamoHttpMethod;
use plamo_http_queries::PlamoHttpQueries;
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
    version: *const c_char
) -> PlamoRequest {
    PlamoRequest {
        scheme: scheme,
        method: method,
        path: path,
        query: &BTreeMap::new(),
        version: version,
        header: &BTreeMap::new(),
        body: &Vec::new(),
    }
}

#[no_mangle]
pub extern fn plamo_request_header_find(plamo_request: *const PlamoRequest, key: *const c_char) -> *const PlamoHttpHeaders {
    unsafe {
        match CStr::from_ptr(key).to_str() {
            Ok(key) => {
                match (*(*plamo_request).header).get(key) {
                    Some(ref headers) => ptr::read(headers) as *const _,
                    None => ptr::null(),
                }
            },
            Err(_) => ptr::null(),
        }
    }
}

#[no_mangle]
pub extern fn plamo_request_query_find(plamo_request: *const PlamoRequest, key: *const c_char) -> *const PlamoHttpQueries {
    unsafe {
        match CStr::from_ptr(key).to_str() {
            Ok(key) => {
                match (*(*plamo_request).query).get(key) {
                    Some(ref queries) => ptr::read(queries) as *const _,
                    None => ptr::null(),
                }
            },
            Err(_) => ptr::null(),
        }
    }
}
