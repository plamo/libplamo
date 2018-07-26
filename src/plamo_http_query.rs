use plamo_string_array::PlamoStringArray;
use std::collections::BTreeMap;
use std::ffi::{CStr, CString};
use std::os::raw::c_char;
use std::ptr;

#[repr(C)]
pub struct PlamoHttpQuery {
    inner: BTreeMap<CString, PlamoStringArray>,
}

#[no_mangle]
pub extern fn plamo_http_query_new() -> *mut PlamoHttpQuery {
    Box::into_raw(Box::new(PlamoHttpQuery {
       inner: BTreeMap::new(),
    }))
}

#[no_mangle]
pub extern fn plamo_http_query_destroy(plamo_http_query: &mut *mut PlamoHttpQuery) {
    if !plamo_http_query.is_null() {
        unsafe {
           Box::from_raw(*plamo_http_query);
        }
        *plamo_http_query = ptr::null_mut();
    }
}

#[no_mangle]
pub extern fn plamo_http_query_get(plamo_http_query: *const PlamoHttpQuery, key: *const c_char) -> *const PlamoStringArray {
    unsafe {
        match (*plamo_http_query).inner.get(CStr::from_ptr(key)) {
            Some(values) => values,
            None => ptr::null(),
        }
    }
}

#[no_mangle]
pub extern fn plamo_http_query_add(plamo_http_query: *mut PlamoHttpQuery, key: *const c_char, value: *const c_char) {
    unsafe {
        match (*plamo_http_query).inner.get_mut(CStr::from_ptr(key)) {
            Some(plamo_string_array) => plamo_string_array.inner.push(CString::from_raw(value as *mut _)),
            None => {
                (*plamo_http_query).inner.insert(CString::from_raw(key as *mut _), PlamoStringArray { inner: vec![CString::from_raw(value as *mut _)]} );
            },
        }
    }
}

#[no_mangle]
pub extern fn plamo_http_query_remove(plamo_http_query: *mut PlamoHttpQuery, key: *const c_char) -> bool {
    unsafe {
        (*plamo_http_query).inner.remove(CStr::from_ptr(key)).is_some()
    }
}
