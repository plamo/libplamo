use plamo_result::PlamoResult;
use plamo_string_array::PlamoStringArray;
use std::collections::BTreeMap;
use std::ffi::{CStr, CString};
use std::os::raw::c_char;

#[repr(C)]
pub struct PlamoHttpQuery {
    inner: BTreeMap<CString, PlamoStringArray>,
}

#[no_mangle]
pub extern fn plamo_http_query_new(plamo_http_query: &mut *mut PlamoHttpQuery) {
    *plamo_http_query = Box::into_raw(Box::new(PlamoHttpQuery {
        inner: BTreeMap::new(),
    }));
}

#[no_mangle]
pub extern fn plamo_http_query_get(plamo_http_query: *const PlamoHttpQuery, key: *const c_char, plamo_string_array: &mut *const PlamoStringArray) -> PlamoResult {
    unsafe {
        match (*plamo_http_query).inner.get(CStr::from_ptr(key)) {
            Some(values) => {
                *plamo_string_array = values;
                PlamoResult::Ok
            },
            None => PlamoResult::NotFound,
        }
    }
}

#[no_mangle]
pub extern fn plamo_http_query_remove(plamo_http_query: *mut PlamoHttpQuery, key: *const c_char) -> PlamoResult {
    unsafe {
        match (*plamo_http_query).inner.remove(CStr::from_ptr(key)) {
            Some(_) => PlamoResult::Ok,
            None => PlamoResult::NotFound,
        }
    }
}
