use std::os::raw::c_uchar;

#[repr(C)]
pub struct PlamoByteArray {
    body: *const c_uchar,
    length: usize,
}

impl PlamoByteArray {
    pub fn new(vec: &Vec<c_uchar>) -> PlamoByteArray {
        PlamoByteArray {
            body: vec.as_ptr(),
            length: vec.len(),
        }
    }
}

#[no_mangle]
pub extern fn plamo_byte_array_new(body: *const c_uchar, length: usize) -> *mut PlamoByteArray {
    Box::into_raw(Box::new(PlamoByteArray {
        body: body,
        length: length,
    }))
}
