use plamo_result::PlamoResult;
use plamo_string_array::PlamoStringArray;
use std::collections::BTreeMap;
use std::ffi::{CStr, CString};
use std::os::raw::c_char;

#[repr(C)]
pub struct PlamoHttpHeader {
    inner: BTreeMap<CString, PlamoStringArray>,
}

impl PlamoHttpHeader {
    pub fn new() -> PlamoHttpHeader {
        PlamoHttpHeader { inner: BTreeMap::new() }
    }
}

#[no_mangle]
pub extern fn plamo_http_header_get(plamo_http_header: *const PlamoHttpHeader, key: *const c_char, plamo_string_array: &mut *const PlamoStringArray) -> PlamoResult {
    unsafe {
        match (*plamo_http_header).inner.get(CStr::from_ptr(key)) {
            Some(values) => {
                *plamo_string_array = values;
                PlamoResult::Ok
            },
            None => PlamoResult::NotFound,
        }
    }
}

#[no_mangle]
pub extern fn plamo_http_header_remove(plamo_http_header: *mut PlamoHttpHeader, key: *const c_char) -> PlamoResult {
    unsafe {
        match (*plamo_http_header).inner.remove(CStr::from_ptr(key)) {
            Some(_) => PlamoResult::Ok,
            None => PlamoResult::NotFound,
        }
    }
}
