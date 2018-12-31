use plamo_string_array::PlamoStringArray;
use std::collections::BTreeMap;
use std::ffi::{CStr, CString};
use std::os::raw::c_char;
use std::ptr;

pub type PlamoHttpQuery = BTreeMap<CString, PlamoStringArray>;

#[no_mangle]
pub extern fn plamo_http_query_new() -> *mut PlamoHttpQuery {
    Box::into_raw(Box::new(PlamoHttpQuery::new()))
}

#[no_mangle]
pub extern fn plamo_http_query_destroy(plamo_http_query: *mut PlamoHttpQuery) {
    unsafe {
       drop(Box::from_raw(plamo_http_query));
    }
}

#[no_mangle]
pub extern fn plamo_http_query_get(plamo_http_query: *const PlamoHttpQuery, key: *const c_char) -> *const PlamoStringArray {
    unsafe {
        match (*plamo_http_query).get(CStr::from_ptr(key)) {
            Some(values) => values,
            None => ptr::null(),
        }
    }
}

#[no_mangle]
pub extern fn plamo_http_query_add(plamo_http_query: *mut PlamoHttpQuery, key: *const c_char, value: *const c_char) {
    unsafe {
        match (*plamo_http_query).get_mut(CStr::from_ptr(key)) {
            Some(plamo_string_array) => {
                if value.is_null() {
                    plamo_string_array.push(CString::new("").unwrap());
                } else {
                    plamo_string_array.push(CString::from_raw(value as *mut _));
                }
            },
            None => {
                if value.is_null() {
                    (*plamo_http_query).insert(CString::from_raw(key as *mut _), vec![CString::new("").unwrap()] );
                } else {
                    (*plamo_http_query).insert(CString::from_raw(key as *mut _), vec![CString::from_raw(value as *mut _)] );
                }
            },
        }
    }
}

#[no_mangle]
pub extern fn plamo_http_query_remove(plamo_http_query: *mut PlamoHttpQuery, key: *const c_char) -> bool {
    unsafe {
        (*plamo_http_query).remove(CStr::from_ptr(key)).is_some()
    }
}
