use std::ffi::{CStr, CString};
use std::os::raw::c_char;

#[repr(C)]
pub struct PlamoHttpHeaders {
    inner: Vec<CString>,
}

#[no_mangle]
pub extern fn plamo_http_headers_for_each(plamo_http_headers: *const PlamoHttpHeaders, callback: extern "C" fn(*const c_char)) {
    unsafe {
        (*plamo_http_headers).inner.iter().for_each(|header| callback(header.as_ptr()));
    }
}

#[no_mangle]
pub extern fn plamo_http_headers_append(plamo_http_headers: *mut PlamoHttpHeaders, value: *const c_char) {
    unsafe {
        (*plamo_http_headers).inner.push(CString::from(CStr::from_ptr(value)));
    }
}
