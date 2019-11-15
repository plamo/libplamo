use crate::plamo_form_data_file::PlamoFormDataFile;
use crate::plamo_string::PlamoString;

#[repr(C)]
pub struct PlamoFormDataField {
    pub text: *mut PlamoString,
    pub file: *mut PlamoFormDataFile,
}
