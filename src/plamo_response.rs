use plamo_http_headers::PlamoHttpHeaders;
use plamo_result::PlamoResult;
use std::collections::BTreeMap;
use std::ffi::{CStr, CString};
use std::os::raw::{c_char, c_uchar, c_uint};
use std::ptr;

#[repr(C)]
pub struct PlamoResponse {
    status_code: c_uint,
    header: *mut BTreeMap<CString, PlamoHttpHeaders>,
    body: *mut Vec<c_uchar>,
}

#[no_mangle]
pub extern fn plamo_response_new(status_code: c_uint, plamo_response: &mut *mut PlamoResponse) -> PlamoResult {
    *plamo_response = Box::into_raw(Box::new(PlamoResponse {
        status_code: status_code,
        header: Box::into_raw(Box::new(BTreeMap::new())),
        body: ptr::null_mut(),
    }));
    PlamoResult::Ok
}

#[no_mangle]
pub extern fn plamo_response_destroy(plamo_response: &mut *mut PlamoResponse) {
    if !plamo_response.is_null() {
        unsafe {
            let plamo_response = Box::from_raw(*plamo_response);
            Box::from_raw(plamo_response.header);
            if !plamo_response.body.is_null() {
                Box::from_raw(plamo_response.body);
            }
        }
        *plamo_response = ptr::null_mut();
    }
}

#[no_mangle]
pub extern fn plamo_response_set_body(plamo_response: *mut PlamoResponse, body: *mut c_uchar, size: usize) -> PlamoResult {
    unsafe {
        if !(*plamo_response).body.is_null() {
            Box::from_raw((*plamo_response).body);
        }
        (*plamo_response).body = Box::into_raw(Box::new(Vec::from_raw_parts(body, size, size)));
    }
    PlamoResult::Ok
}

#[no_mangle]
pub extern fn plamo_response_find_headers(plamo_response: *mut PlamoResponse, key: *const c_char, plamo_http_headers: &mut *const PlamoHttpHeaders) -> PlamoResult {
    unsafe {
        match (*(*plamo_response).header).get(CStr::from_ptr(key)) {
            Some(headers) => {
                *plamo_http_headers = headers;
                PlamoResult::Ok
            },
            None => PlamoResult::NotFound,
        }
    }
}
