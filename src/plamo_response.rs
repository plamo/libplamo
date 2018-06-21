use plamo_byte_array::PlamoByteArray;
use plamo_http_header::PlamoHttpHeader;
use std::os::raw::c_uint;
use std::ptr;

#[repr(C)]
pub struct PlamoResponse {
    status_code: c_uint,
    header: *const PlamoHttpHeader,
    body: *const PlamoByteArray,
}

#[no_mangle]
pub extern fn plamo_response_new(status_code: c_uint, header: *const PlamoHttpHeader, body: *const PlamoByteArray) -> *mut PlamoResponse {
    Box::into_raw(Box::new(PlamoResponse {
        status_code: status_code,
        header: header,
        body: body,
    }))
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
    unsafe { (*plamo_response).status_code }
}
