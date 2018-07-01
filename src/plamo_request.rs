use plamo_byte_array::PlamoByteArray;
use plamo_http_header::PlamoHttpHeader;
use plamo_http_method::PlamoHttpMethod;
use plamo_http_query::PlamoHttpQuery;
use plamo_scheme::PlamoScheme;
use plamo_string::{PlamoString, plamo_string_new};
use std::os::raw::c_char;
use std::ptr;

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
    query: *const PlamoHttpQuery,
    header: *const PlamoHttpHeader,
    body: *const PlamoByteArray,
) -> *mut PlamoRequest {
    Box::into_raw(Box::new(PlamoRequest {
        method: method,
        scheme: scheme,
        path: plamo_string_new(path),
        version: plamo_string_new(version),
        query: query,
        header: header,
        body: body,
    }))
}

#[no_mangle]
pub extern fn plamo_request_destroy(plamo_request: &mut *mut PlamoRequest) {
    if !plamo_request.is_null() {
        unsafe {
            Box::from_raw((**plamo_request).path as *mut PlamoString);
            (**plamo_request).path = ptr::null();
            Box::from_raw((**plamo_request).version as *mut PlamoString);
            (**plamo_request).version = ptr::null();
            Box::from_raw((**plamo_request).query as *mut PlamoHttpQuery);
            (**plamo_request).query = ptr::null();
            Box::from_raw((**plamo_request).header as *mut PlamoHttpHeader);
            (**plamo_request).header = ptr::null();
            Box::from_raw((**plamo_request).body as *mut PlamoByteArray);
            (**plamo_request).body = ptr::null();
            Box::from_raw(*plamo_request);
        }
        *plamo_request = ptr::null_mut();
    }
}
