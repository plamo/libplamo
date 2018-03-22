use plamo_http_headers::PlamoHttpHeaders;
use plamo_result::PlamoResult;
use std::collections::BTreeMap;
use std::ffi::CString;
use std::os::raw::{c_uchar, c_uint};
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
        body: Box::into_raw(Box::new(Vec::new())),
    }));
    PlamoResult::Ok
}

#[no_mangle]
pub extern fn plamo_response_destroy(plamo_response: &mut *mut PlamoResponse) {
    if !plamo_response.is_null() {
        unsafe {
            let plamo_response = Box::from_raw(*plamo_response);
            Box::from_raw(plamo_response.header);
            Box::from_raw(plamo_response.body);
        }
        *plamo_response = ptr::null_mut();
    }
}
