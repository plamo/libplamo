use std::ffi::CString;
use std::os::raw::c_char;
use std::ptr;

#[repr(C)]
pub struct PlamoStringArray {
    pub(crate) inner: Vec<CString>,
}

#[no_mangle]
pub extern fn plamo_string_array_new() -> *mut PlamoStringArray {
    Box::into_raw(Box::new(PlamoStringArray {
        inner: Vec::new(),
    }))
}

#[no_mangle]
pub extern fn plamo_string_array_for_each(plamo_string_array: *const PlamoStringArray, callback: extern fn(*const c_char)) {
    unsafe {
        (*plamo_string_array).inner.iter().for_each(|header| callback(header.as_ptr()));
    }
}

#[no_mangle]
pub extern fn plamo_string_array_get_at(plamo_string_array: *const PlamoStringArray, index: usize) -> *const c_char {
    unsafe {
        match (*plamo_string_array).inner.get(index) {
            Some(v) => v.as_ptr(),
            None => ptr::null(),
        }
    }
}

#[no_mangle]
pub extern fn plamo_string_array_get_first(plamo_string_array: *const PlamoStringArray) -> *const c_char {
    unsafe {
        match (*plamo_string_array).inner.first() {
            Some(v) => v.as_ptr(),
            None => ptr::null(),
        }
    }
}

#[no_mangle]
pub extern fn plamo_string_array_get_last(plamo_string_array: *const PlamoStringArray) -> *const c_char {
    unsafe {
        match (*plamo_string_array).inner.last() {
            Some(v) => v.as_ptr(),
            None => ptr::null(),
        }
    }
}

#[no_mangle]
pub extern fn plamo_string_array_add(plamo_string_array: *mut PlamoStringArray, value: *const c_char) {
    unsafe {
        (*plamo_string_array).inner.push(CString::from_raw(value as *mut _));
    }
}

#[no_mangle]
pub extern fn plamo_string_array_remove_at(plamo_string_array: *mut PlamoStringArray, index: usize) -> bool {
    unsafe {
        (*plamo_string_array).inner.get(index).map(|_| (*plamo_string_array).inner.remove(index)).is_some()
    }
}
