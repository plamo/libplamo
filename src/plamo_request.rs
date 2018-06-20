use plamo_byte_array::PlamoByteArray;
use plamo_http_header::PlamoHttpHeader;
use plamo_http_method::PlamoHttpMethod;
use plamo_http_query::PlamoHttpQuery;
use std::ffi::CString;
use std::os::raw::c_char;
use std::ptr;

#[repr(C)]
pub struct PlamoRequest {
    method: PlamoHttpMethod,
    scheme: CString,
    path: CString,
    version: CString,
    query: *const PlamoHttpQuery,
    header: *const PlamoHttpHeader,
    body: *const PlamoByteArray,
}

#[no_mangle]
pub extern fn plamo_request_new(
    method: PlamoHttpMethod,
    scheme: *const c_char,
    path: *const c_char,
    version: *const c_char,
    query: *const PlamoHttpQuery,
    header: *const PlamoHttpHeader,
    body: *const PlamoByteArray,
) -> *mut PlamoRequest {
    Box::into_raw(Box::new(PlamoRequest {
        method: method,
        scheme: unsafe { CString::from_raw(scheme as *mut _) },
        path: unsafe { CString::from_raw(path as *mut _) },
        version: unsafe { CString::from_raw(version as *mut _) },
        query: query,
        header: header,
        body: body,
    }))
}

#[no_mangle]
pub extern fn plamo_request_destroy(plamo_request: &mut *mut PlamoRequest) {
    if !plamo_request.is_null() {
        unsafe {
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
