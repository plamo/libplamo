use crate::plamo_byte_array::PlamoByteArray;
use crate::plamo_http_header::{PlamoHttpHeader, plamo_http_header_new};
use crate::plamo_http_query::PlamoHttpQuery;
use crate::plamo_http_version::PlamoHttpVersion;
use crate::plamo_scheme::PlamoScheme;
use crate::plamo_string::{PlamoString, plamo_string_new};
use std::os::raw::c_char;

#[repr(C)]
pub struct PlamoRequest {
    method: *const PlamoString,
    scheme: PlamoScheme,
    path: *const PlamoString,
    version: PlamoHttpVersion,
    query: *const PlamoHttpQuery,
    header: *const PlamoHttpHeader,
    body: *const PlamoByteArray,
}

#[no_mangle]
pub extern fn plamo_request_new(
    method: *const c_char,
    scheme: PlamoScheme,
    path: *const c_char,
    version: PlamoHttpVersion,
    query: *const PlamoHttpQuery,
    body: *const PlamoByteArray,
) -> *mut PlamoRequest {
    Box::into_raw(Box::new(PlamoRequest {
        method: plamo_string_new(method),
        scheme: scheme,
        path: plamo_string_new(path),
        version: version,
        query: query,
        header: plamo_http_header_new(),
        body: body,
    }))
}

#[no_mangle]
pub extern fn plamo_request_destroy(plamo_request: *mut PlamoRequest) {
    unsafe {
        drop(Box::from_raw((*plamo_request).method as *mut PlamoString));
        drop(Box::from_raw((*plamo_request).path as *mut PlamoString));
        drop(Box::from_raw((*plamo_request).query as *mut PlamoHttpQuery));
        drop(Box::from_raw((*plamo_request).header as *mut PlamoHttpHeader));
        drop(Box::from_raw((*plamo_request).body as *mut PlamoByteArray));
        drop(Box::from_raw(plamo_request));
    }
}
