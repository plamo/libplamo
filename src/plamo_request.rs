use plamo_byte_array::PlamoByteArray;
use plamo_http_header::PlamoHttpHeader;
use plamo_http_method::PlamoHttpMethod;
use plamo_http_query::PlamoHttpQuery;
use std::ffi::CString;
use std::os::raw::{c_char, c_uchar};
use std::ptr;

#[repr(C)]
pub struct PlamoRequest {
    method: PlamoHttpMethod,
    scheme: CString,
    path: CString,
    version: CString,
    query: *const PlamoHttpQuery,
    header: PlamoHttpHeader,
    body: Vec<c_uchar>,
}

#[no_mangle]
pub extern fn plamo_request_new(
    method: PlamoHttpMethod,
    scheme: *const c_char,
    path: *const c_char,
    version: *const c_char,
    query: *const PlamoHttpQuery,
) -> *mut PlamoRequest {
    Box::into_raw(Box::new(PlamoRequest {
        method: method,
        scheme: unsafe { CString::from_raw(scheme as *mut _) },
        path: unsafe { CString::from_raw(path as *mut _) },
        version: unsafe { CString::from_raw(version as *mut _) },
        query: query,
        header: PlamoHttpHeader::new(),
        body: Vec::new(),
    }))
}

#[no_mangle]
pub extern fn plamo_request_destroy(plamo_request: &mut *mut PlamoRequest) {
    if !plamo_request.is_null() {
        unsafe {
            Box::from_raw((**plamo_request).query as *mut PlamoHttpQuery);
            Box::from_raw(*plamo_request);
        }
        *plamo_request = ptr::null_mut();
    }
}

#[no_mangle]
pub extern fn plamo_request_get_method(plamo_request: *const PlamoRequest) -> PlamoHttpMethod {
    unsafe { (*plamo_request).method }
}

#[no_mangle]
pub extern fn plamo_request_get_scheme(plamo_request: *const PlamoRequest) -> *const c_char {
    unsafe { (*plamo_request).scheme.as_ptr() }
}

#[no_mangle]
pub extern fn plamo_request_get_path(plamo_request: *const PlamoRequest) -> *const c_char {
    unsafe { (*plamo_request).path.as_ptr() }
}

#[no_mangle]
pub extern fn plamo_request_get_version(plamo_request: *const PlamoRequest) -> *const c_char {
    unsafe { (*plamo_request).version.as_ptr() }
}

#[no_mangle]
pub extern fn plamo_request_get_header(plamo_request: *mut PlamoRequest) -> *mut PlamoHttpHeader {
    unsafe { &mut (*plamo_request).header }
}

#[no_mangle]
pub extern fn plamo_request_get_body(plamo_request: *const PlamoRequest) -> PlamoByteArray {
    unsafe { PlamoByteArray::new(&(*plamo_request).body) }
}

#[no_mangle]
pub extern fn plamo_request_set_body(plamo_request: *mut PlamoRequest, body: *mut c_uchar, size: usize) {
    unsafe {
        (*plamo_request).body = Vec::from_raw_parts(body, size, size);
    }
}
