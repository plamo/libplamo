use std::ffi::CString;
use std::os::raw::c_char;
use std::ptr;

#[repr(C)]
pub struct PlamoString {
    inner: CString,
}

#[no_mangle]
pub extern fn plamo_string_new(value: *const c_char) -> *const PlamoString {
    Box::into_raw(Box::new(PlamoString {
        inner: unsafe { CString::from_raw(value as *mut c_char) },
    }))
}

#[no_mangle]
pub extern fn plamo_string_get_char(plamo_string: *const PlamoString) -> *const c_char {
    unsafe {
        (*plamo_string).inner.as_ptr()
    }
}

#[no_mangle]
pub extern fn plamo_string_destroy(plamo_string: &mut *mut PlamoString) {
    if !plamo_string.is_null() {
        unsafe { Box::from_raw(*plamo_string); }
        *plamo_string = ptr::null_mut();
    }
}
