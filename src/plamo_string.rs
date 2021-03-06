use std::ffi::{CStr, CString};
use std::os::raw::c_char;

pub type PlamoString = CString;

#[no_mangle]
pub extern fn plamo_string_new(value: *const c_char) -> *mut PlamoString {
    unsafe {
        Box::into_raw(Box::new(CStr::from_ptr(value).to_owned()))
    }
}

#[no_mangle]
pub extern fn plamo_string_destroy(plamo_string: *mut PlamoString) {
    unsafe {
        drop(Box::from_raw(plamo_string));
    }
}

#[no_mangle]
pub extern fn plamo_string_get_char(plamo_string: *const PlamoString) -> *const c_char {
    unsafe {
        (*plamo_string).as_ptr()
    }
}
