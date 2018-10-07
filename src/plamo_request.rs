use plamo_byte_array::PlamoByteArray;
use plamo_http_header::{PlamoHttpHeader, plamo_http_header_new};
use plamo_http_method::PlamoHttpMethod;
use plamo_http_query::{PlamoHttpQuery, plamo_http_query_new};
use plamo_scheme::PlamoScheme;
use plamo_string::{PlamoString, plamo_string_new};
use std::os::raw::c_char;

#[repr(C)]
pub struct PlamoRequest {
    method: PlamoHttpMethod,
    scheme: PlamoScheme,
    path: *const PlamoString,
    version: *const PlamoString,
    query: *const PlamoHttpQuery,
    header: *const PlamoHttpHeader,
    body: *const PlamoByteArray,
}

#[no_mangle]
pub extern fn plamo_request_new(
    method: PlamoHttpMethod,
    scheme: PlamoScheme,
    path: *const c_char,
    version: *const c_char,
    body: *const PlamoByteArray,
) -> *mut PlamoRequest {
    Box::into_raw(Box::new(PlamoRequest {
        method: method,
        scheme: scheme,
        path: plamo_string_new(path),
        version: plamo_string_new(version),
        query: plamo_http_query_new(),
        header: plamo_http_header_new(),
        body: body,
    }))
}

#[no_mangle]
pub extern fn plamo_request_destroy(plamo_request: *mut PlamoRequest) {
    unsafe {
        drop(Box::from_raw((*plamo_request).path as *mut PlamoString));
        drop(Box::from_raw((*plamo_request).version as *mut PlamoString));
        drop(Box::from_raw((*plamo_request).query as *mut PlamoHttpQuery));
        drop(Box::from_raw((*plamo_request).header as *mut PlamoHttpHeader));
        drop(Box::from_raw((*plamo_request).body as *mut PlamoByteArray));
        drop(Box::from_raw(plamo_request));
    }
}
