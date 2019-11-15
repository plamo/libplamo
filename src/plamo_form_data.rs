use crate::plamo_form_data_field::PlamoFormDataField;
use crate::plamo_form_data_field_array::PlamoFormDataFieldArray;
use crate::plamo_form_data_file::PlamoFormDataFile;
use crate::plamo_string::PlamoString;
use crate::plamo_request::PlamoRequest;
use multipart::server::Multipart;
use std::collections::BTreeMap;
use std::ffi::CStr;
use std::io::Read;
use std::os::raw::c_char;

pub type PlamoFormData = BTreeMap<PlamoString, PlamoFormDataFieldArray>;

#[no_mangle]
pub extern fn plamo_form_data_new(plamo_request: *const PlamoRequest) -> *mut PlamoFormData {
    let mut plamo_form_data = PlamoFormData::new();
    if let Ok(mut multipart) = Multipart::from_request(unsafe { &*plamo_request }) {
        multipart.foreach_entry(|mut entry| {
            let mut buf = Vec::new();
            entry.data.read_to_end(&mut buf).unwrap();

            let field = if let Some(content_type) = entry.headers.content_type {
                let file = PlamoFormDataFile::new(
                    PlamoString::new(content_type.to_string()).unwrap(),
                    entry.headers.filename.map(|v| PlamoString::new(v).unwrap()),
                    buf,
                );

                PlamoFormDataField {
                    text: std::ptr::null_mut(),
                    file: Box::into_raw(Box::new(file)),
                }
            } else {
                PlamoFormDataField {
                    text: Box::into_raw(Box::new(PlamoString::new(buf).unwrap())),
                    file: std::ptr::null_mut(),
                }
            };

            match plamo_form_data.get_mut(&PlamoString::new(entry.headers.name.to_string()).unwrap()) {
                Some(vec) => {
                    vec.push(field);
                },
                None => {
                    plamo_form_data.insert(PlamoString::new(entry.headers.name.to_string()).unwrap(), vec![field]);
                }
            }
        }).unwrap();

        Box::into_raw(Box::new(plamo_form_data))
    } else {
        std::ptr::null_mut()
    }
}

#[no_mangle]
pub extern fn plamo_form_data_destroy(plamo_form_data: *mut PlamoFormData) {
    unsafe {
        (*plamo_form_data).values().for_each(|values| {
            values.iter().for_each(|value| {
                if value.text != std::ptr::null_mut() {
                    drop(Box::from_raw(value.text));
                } else {
                    drop(Box::from_raw(value.file));
                }
            });
        });
       drop(Box::from_raw(plamo_form_data));
    }
}

#[no_mangle]
pub extern fn plamo_form_data_for_each(plamo_form_data: *const PlamoFormData, callback: extern fn(*const c_char, *const PlamoFormDataField)) {
    unsafe {
        (*plamo_form_data).iter().for_each(|(key, values)| {
            values.iter().for_each(|value| {
                callback(key.as_ptr(), value);
            });
        });
    }
}

#[no_mangle]
pub extern fn plamo_form_data_get(plamo_form_data: *const PlamoFormData, key: *const c_char) -> *const PlamoFormDataFieldArray {
    unsafe {
        match (*plamo_form_data).get(CStr::from_ptr(key)) {
            Some(values) => values,
            None => std::ptr::null()
        }
    }
}
