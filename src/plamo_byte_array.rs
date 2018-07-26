use std::os::raw::c_uchar;

pub type PlamoByteArray = Vec<c_uchar>;

#[no_mangle]
pub extern fn plamo_byte_array_new(body: *const c_uchar, length: usize) -> *mut PlamoByteArray {
    unsafe { Box::into_raw(Box::new(Vec::from_raw_parts(body as *mut _, length, length))) }
}

#[no_mangle]
pub extern fn plamo_byte_array_get_body(plamo_byte_array: *const PlamoByteArray) -> *const c_uchar {
    unsafe { (*plamo_byte_array).as_ptr() }
}

#[no_mangle]
pub extern fn plamo_byte_array_get_body_size(plamo_byte_array: *const PlamoByteArray) -> usize {
    unsafe { (*plamo_byte_array).len() }
}
