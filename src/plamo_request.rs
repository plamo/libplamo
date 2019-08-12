use crate::plamo_byte_array::PlamoByteArray;
use crate::plamo_http_header::PlamoHttpHeader;
use crate::plamo_http_query::PlamoHttpQuery;
use crate::plamo_http_version::PlamoHttpVersion;
use crate::plamo_scheme::PlamoScheme;
use crate::plamo_string::{PlamoString, plamo_string_new};
use std::os::raw::c_char;

#[repr(C)]
pub struct PlamoRequest {
    scheme: PlamoScheme,
    version: PlamoHttpVersion,
    method: *mut PlamoString,
    path: *mut PlamoString,
    query: *mut PlamoHttpQuery,
    header: *mut PlamoHttpHeader,
    body: *mut PlamoByteArray,
}

#[no_mangle]
pub extern fn plamo_request_new(
    scheme: PlamoScheme,
    version: PlamoHttpVersion,
    method: *mut c_char,
    path: *mut c_char,
    query: *mut PlamoHttpQuery,
    header: *mut PlamoHttpHeader,
    body: *mut PlamoByteArray,
) -> *mut PlamoRequest {
    Box::into_raw(Box::new(PlamoRequest {
        scheme,
        version,
        query,
        header,
        body,
        method: plamo_string_new(method),
        path: plamo_string_new(path),
    }))
}

#[no_mangle]
pub extern fn plamo_request_destroy(plamo_request: *mut PlamoRequest) {
    unsafe {
        drop(Box::from_raw((*plamo_request).method as *mut PlamoString));
        drop(Box::from_raw((*plamo_request).path as *mut PlamoString));
        drop(Box::from_raw(plamo_request));
    }
}
