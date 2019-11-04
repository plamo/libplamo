use crate::plamo_string::PlamoString;
use crate::plamo_string_array::PlamoStringArray;
use std::collections::BTreeMap;
use std::ffi::CStr;
use std::os::raw::c_char;
use std::ptr;

pub type PlamoHttpHeader = BTreeMap<PlamoString, PlamoStringArray>;

#[no_mangle]
pub extern fn plamo_http_header_new() -> *mut PlamoHttpHeader {
    Box::into_raw(Box::new(PlamoHttpHeader::new()))
}

#[no_mangle]
pub extern fn plamo_http_header_destroy(plamo_http_header: *mut PlamoHttpHeader) {
    unsafe {
       drop(Box::from_raw(plamo_http_header));
    }
}

#[no_mangle]
pub extern fn plamo_http_header_for_each(plamo_http_header: *mut PlamoHttpHeader, callback: extern fn(*const c_char, *const c_char)) {
    unsafe {
        (*plamo_http_header).iter().for_each(|(key, values)| {
            values.iter().for_each(|value| callback(key.as_ptr(), value.as_ptr()));
        });
    }
}

#[no_mangle]
pub extern fn plamo_http_header_get(plamo_http_header: *mut PlamoHttpHeader, key: *const c_char) -> *mut PlamoStringArray {
    unsafe {
        match (*plamo_http_header).get_mut(CStr::from_ptr(key)) {
            Some(values) => values,
            None => ptr::null_mut(),
        }
    }
}

#[no_mangle]
pub extern fn plamo_http_header_add(plamo_http_header: *mut PlamoHttpHeader, key: *const c_char, value: *const c_char) {
    unsafe {
        if !key.is_null() && !value.is_null() {
            match (*plamo_http_header).get_mut(CStr::from_ptr(key)) {
                Some(plamo_string_array) => plamo_string_array.push(CStr::from_ptr(value).to_owned()),
                None => {
                    (*plamo_http_header).insert(CStr::from_ptr(key).to_owned(), vec![CStr::from_ptr(value).to_owned()] );
                },
            }
        }
    }
}

#[no_mangle]
pub extern fn plamo_http_header_remove(plamo_http_header: *mut PlamoHttpHeader, key: *const c_char) -> bool {
    unsafe {
        (*plamo_http_header).remove(CStr::from_ptr(key)).is_some()
    }
}
