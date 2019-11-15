use crate::plamo_string::PlamoString;
use std::os::raw::{c_char, c_uchar};

pub struct PlamoFormDataFile {
  content_type: PlamoString,
  file_name: Option<PlamoString>,
  body: Vec<c_uchar>,
}

impl PlamoFormDataFile {
  pub fn new(content_type: PlamoString, file_name: Option<PlamoString>, body: Vec<c_uchar>) -> PlamoFormDataFile {
    PlamoFormDataFile { content_type, file_name, body }
  }
}

#[no_mangle]
pub extern fn plamo_form_data_file_get_content_type(plamo_form_data_file: *const PlamoFormDataFile) -> *const c_char {
    unsafe {
        (*plamo_form_data_file).content_type.as_ptr()
    }
}

#[no_mangle]
pub extern fn plamo_form_data_file_get_file_name(plamo_form_data_file: *const PlamoFormDataFile) -> *const c_char {
    unsafe {
        match (*plamo_form_data_file).file_name.as_ref() {
            Some(file_name) => file_name.as_ptr(),
            None => std::ptr::null(),
        }
    }
}

#[no_mangle]
pub extern fn plamo_form_data_file_get_body(plamo_form_data_file: *const PlamoFormDataFile) -> *const c_uchar {
    unsafe {
        (*plamo_form_data_file).body.as_ptr()
    }
}

#[no_mangle]
pub extern fn plamo_form_data_file_get_body_size(plamo_form_data_file: *const PlamoFormDataFile) -> usize {
    unsafe {
        (*plamo_form_data_file).body.len()
    }
}
