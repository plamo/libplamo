use plamo_byte_array::PlamoByteArray;
use plamo_http_header::{PlamoHttpHeader, plamo_http_header_new};
use std::os::raw::c_uint;
use std::ptr;

#[repr(C)]
pub struct PlamoResponse {
    status_code: c_uint,
    header: *const PlamoHttpHeader,
    body: *const PlamoByteArray,
}

#[no_mangle]
pub extern fn plamo_response_new() -> *mut PlamoResponse {
    Box::into_raw(Box::new(PlamoResponse {
        status_code: 200,
        header: plamo_http_header_new(),
        body: ptr::null(),
    }))
}

#[no_mangle]
pub extern fn plamo_response_destroy(plamo_response: &mut *mut PlamoResponse) {
    if !plamo_response.is_null() {
        unsafe {
            if !(**plamo_response).header.is_null() {
                Box::from_raw((**plamo_response).header as *mut PlamoHttpHeader);
                (**plamo_response).header = ptr::null();
            }
            if !(**plamo_response).body.is_null() {
                Box::from_raw((**plamo_response).body as *mut PlamoByteArray);
                (**plamo_response).body = ptr::null();
            }
            Box::from_raw(*plamo_response);
        }
        *plamo_response = ptr::null_mut();
    }
}
