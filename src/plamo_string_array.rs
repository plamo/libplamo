use plamo_result::PlamoResult;
use std::ffi::{CStr, CString};
use std::os::raw::c_char;

#[repr(C)]
pub struct PlamoStringArray {
    inner: Vec<CString>,
}

#[no_mangle]
pub extern fn plamo_string_array_new(plamo_string_array: &mut *mut PlamoStringArray) {
    *plamo_string_array = Box::into_raw(Box::new(PlamoStringArray {
        inner: Vec::new(),
    }));
}

#[no_mangle]
pub extern fn plamo_string_array_for_each(plamo_string_array: *const PlamoStringArray, callback: extern fn(*const c_char)) {
    unsafe {
        (*plamo_string_array).inner.iter().for_each(|header| callback(header.as_ptr()));
    }
}

#[no_mangle]
pub extern fn plamo_string_array_get_at(plamo_string_array: *const PlamoStringArray, index: usize, value: *mut *const c_char) -> PlamoResult {
    unsafe {
        match (*plamo_string_array).inner.get(index) {
            Some(v) => {
                *value = v.as_ptr();
                PlamoResult::Ok
            },
            None => PlamoResult::NotFound,
        }
    }
}

#[no_mangle]
pub extern fn plamo_string_array_get_first(plamo_string_array: *const PlamoStringArray, value: *mut *const c_char) -> PlamoResult {
    unsafe {
        match (*plamo_string_array).inner.first() {
            Some(v) => {
                *value = v.as_ptr();
                PlamoResult::Ok
            },
            None => PlamoResult::NotFound,
        }
    }
}

#[no_mangle]
pub extern fn plamo_string_array_get_last(plamo_string_array: *const PlamoStringArray, value: *mut *const c_char) -> PlamoResult {
    unsafe {
        match (*plamo_string_array).inner.last() {
            Some(v) => {
                *value = v.as_ptr();
                PlamoResult::Ok
            },
            None => PlamoResult::NotFound,
        }
    }
}

#[no_mangle]
pub extern fn plamo_string_array_add(plamo_string_array: *mut PlamoStringArray, value: *const c_char) {
    unsafe {
        (*plamo_string_array).inner.push(CString::from(CStr::from_ptr(value)));
    }
}

#[no_mangle]
pub extern fn plamo_string_array_remove_at(plamo_string_array: *mut PlamoStringArray, index: usize) -> PlamoResult {
    unsafe {
        match (*plamo_string_array).inner.get(index) {
            Some(_) => {
                (*plamo_string_array).inner.remove(index);
                PlamoResult::Ok
            },
            None => PlamoResult::NotFound,
        }
    }
}
