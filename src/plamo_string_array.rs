use std::ffi::CStr;
use std::os::raw::c_char;
use std::ptr;
use crate::plamo_string::PlamoString;

pub type PlamoStringArray = Vec<PlamoString>;

#[no_mangle]
pub extern fn plamo_string_array_new() -> *mut PlamoStringArray {
    Box::into_raw(Box::new(PlamoStringArray::new()))
}

#[no_mangle]
pub extern fn plamo_string_array_destroy(plamo_string_array: *mut PlamoStringArray) {
    unsafe {
        drop(Box::from_raw(plamo_string_array));
    }
}

#[no_mangle]
pub extern fn plamo_string_array_for_each(plamo_string_array: *const PlamoStringArray, callback: extern fn(*const c_char)) {
    unsafe {
        (*plamo_string_array).iter().for_each(|value| callback(value.as_ptr()));
    }
}

#[no_mangle]
pub extern fn plamo_string_array_get_at(plamo_string_array: *const PlamoStringArray, index: usize) -> *const c_char {
    unsafe {
        match (*plamo_string_array).get(index) {
            Some(v) => v.as_ptr(),
            None => ptr::null(),
        }
    }
}

#[no_mangle]
pub extern fn plamo_string_array_get_first(plamo_string_array: *const PlamoStringArray) -> *const c_char {
    unsafe {
        match (*plamo_string_array).first() {
            Some(v) => v.as_ptr(),
            None => ptr::null(),
        }
    }
}

#[no_mangle]
pub extern fn plamo_string_array_get_last(plamo_string_array: *const PlamoStringArray) -> *const c_char {
    unsafe {
        match (*plamo_string_array).last() {
            Some(v) => v.as_ptr(),
            None => ptr::null(),
        }
    }
}

#[no_mangle]
pub extern fn plamo_string_array_add(plamo_string_array: *mut PlamoStringArray, value: *const c_char) {
    unsafe {
        (*plamo_string_array).push(CStr::from_ptr(value).to_owned());
    }
}

#[no_mangle]
pub extern fn plamo_string_array_remove_at(plamo_string_array: *mut PlamoStringArray, index: usize) -> bool {
    unsafe {
        (*plamo_string_array).get(index).map(|_| (*plamo_string_array).remove(index)).is_some()
    }
}
