use plamo_http_header::{PlamoHttpHeader, plamo_http_header_new};
use plamo_result::PlamoResult;
use std::os::raw::{c_uchar, c_uint};
use std::ptr;

#[repr(C)]
pub struct PlamoResponse {
    status_code: c_uint,
    header: *mut PlamoHttpHeader,
    body: *mut Vec<c_uchar>,
}

#[no_mangle]
pub extern fn plamo_response_new(status_code: c_uint, plamo_response: &mut *mut PlamoResponse) -> PlamoResult {
    let mut header = ptr::null_mut();
    plamo_http_header_new(&mut header);
    *plamo_response = Box::into_raw(Box::new(PlamoResponse {
        status_code: status_code,
        header: header,
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
