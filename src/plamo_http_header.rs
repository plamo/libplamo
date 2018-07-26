use plamo_string_array::PlamoStringArray;
use std::collections::BTreeMap;
use std::ffi::{CStr, CString};
use std::os::raw::c_char;
use std::ptr;

pub type PlamoHttpHeader = BTreeMap<CString, PlamoStringArray>;

#[no_mangle]
pub extern fn plamo_http_header_new() -> *mut PlamoHttpHeader {
    Box::into_raw(Box::new(BTreeMap::new()))
}

#[no_mangle]
pub extern fn plamo_http_header_destroy(plamo_http_header: &mut *mut PlamoHttpHeader) {
    if !plamo_http_header.is_null() {
        unsafe {
           Box::from_raw(*plamo_http_header);
        }
        *plamo_http_header = ptr::null_mut();
    }
}

#[no_mangle]
pub extern fn plamo_http_header_get(plamo_http_header: *const PlamoHttpHeader, key: *const c_char) -> *const PlamoStringArray {
    unsafe {
        match (*plamo_http_header).get(CStr::from_ptr(key)) {
            Some(values) => values,
            None => ptr::null(),
        }
    }
}

#[no_mangle]
pub extern fn plamo_http_header_add(plamo_http_header: *mut PlamoHttpHeader, key: *const c_char, value: *const c_char) {
    unsafe {
        match (*plamo_http_header).get_mut(CStr::from_ptr(key)) {
            Some(plamo_string_array) => plamo_string_array.push(CString::from_raw(value as *mut _)),
            None => {
                (*plamo_http_header).insert(CString::from_raw(key as *mut _), vec![CString::from_raw(value as *mut _)] );
            },
        }
    }
}

#[no_mangle]
pub extern fn plamo_http_header_remove(plamo_http_header: *mut PlamoHttpHeader, key: *const c_char) -> bool {
    unsafe {
        (*plamo_http_header).remove(CStr::from_ptr(key)).is_some()
    }
}
