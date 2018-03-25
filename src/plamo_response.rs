use plamo_http_header::PlamoHttpHeader;
use plamo_result::PlamoResult;
use std::os::raw::{c_uchar, c_uint};
use std::ptr;

#[repr(C)]
pub struct PlamoResponse {
    status_code: c_uint,
    header: PlamoHttpHeader,
    body: Vec<c_uchar>,
}

#[no_mangle]
pub extern fn plamo_response_new(status_code: c_uint, plamo_response: &mut *mut PlamoResponse) -> PlamoResult {
    *plamo_response = Box::into_raw(Box::new(PlamoResponse {
        status_code: status_code,
        header: PlamoHttpHeader::new(),
        body: Vec::new(),
    }));
    PlamoResult::Ok
}

#[no_mangle]
pub extern fn plamo_response_destroy(plamo_response: &mut *mut PlamoResponse) {
    if !plamo_response.is_null() {
        unsafe { Box::from_raw(*plamo_response); }
        *plamo_response = ptr::null_mut();
    }
}

#[no_mangle]
pub extern fn plamo_response_set_body(plamo_response: *mut PlamoResponse, body: *mut c_uchar, size: usize) -> PlamoResult {
    unsafe {
        (*plamo_response).body = Vec::from_raw_parts(body, size, size);
    }
    PlamoResult::Ok
}
