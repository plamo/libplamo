use plamo_http_header::PlamoHttpHeader;
use std::os::raw::{c_uchar, c_uint};
use std::ptr;

#[repr(C)]
pub struct PlamoResponse {
    status_code: c_uint,
    header: PlamoHttpHeader,
    body: Vec<c_uchar>,
}

#[no_mangle]
pub extern fn plamo_response_new(status_code: c_uint, plamo_response: &mut *mut PlamoResponse) {
    *plamo_response = Box::into_raw(Box::new(PlamoResponse {
        status_code: status_code,
        header: PlamoHttpHeader::new(),
        body: Vec::new(),
    }));
}

#[no_mangle]
pub extern fn plamo_response_destroy(plamo_response: &mut *mut PlamoResponse) {
    if !plamo_response.is_null() {
        unsafe { Box::from_raw(*plamo_response); }
        *plamo_response = ptr::null_mut();
    }
}

#[no_mangle]
pub extern fn plamo_response_get_status_code(plamo_response: *const PlamoResponse) -> c_uint {
    unsafe {
        (*plamo_response).status_code
    }
}

#[no_mangle]
pub extern fn plamo_response_get_header(plamo_response: *mut PlamoResponse) -> *mut PlamoHttpHeader {
    unsafe {
        &mut (*plamo_response).header
    }
}

#[no_mangle]
pub extern fn plamo_response_set_body(plamo_response: *mut PlamoResponse, body: *mut c_uchar, size: usize) {
    unsafe {
        (*plamo_response).body = Vec::from_raw_parts(body, size, size);
    }
}
