use crate::plamo_byte_array::PlamoByteArray;
use crate::plamo_http_header::{PlamoHttpHeader, plamo_http_header_new};
use std::os::raw::c_uint;
use std::ptr;

#[repr(C)]
pub struct PlamoResponse {
    status_code: c_uint,
    header: *mut PlamoHttpHeader,
    body: *mut PlamoByteArray,
}

#[no_mangle]
pub extern fn plamo_response_new() -> *mut PlamoResponse {
    Box::into_raw(Box::new(PlamoResponse {
        status_code: 200,
        header: plamo_http_header_new(),
        body: ptr::null_mut(),
    }))
}

#[no_mangle]
pub extern fn plamo_response_destroy(plamo_response: *mut PlamoResponse) {
    unsafe {
        drop(Box::from_raw((*plamo_response).header));
        if !(*plamo_response).body.is_null() {
            drop(Box::from_raw((*plamo_response).body));
        }
        drop(Box::from_raw(plamo_response));
    }
}
