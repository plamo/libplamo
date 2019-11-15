use crate::plamo_form_data_field::PlamoFormDataField;

pub type PlamoFormDataFieldArray = Vec<PlamoFormDataField>;

#[no_mangle]
pub extern fn plamo_form_data_field_array_length(plamo_form_data_field_array: *const PlamoFormDataFieldArray) -> usize {
    unsafe {
        (*plamo_form_data_field_array).len()
    }
}

#[no_mangle]
pub extern fn plamo_form_data_field_array_for_each(plamo_form_data_field_array: *const PlamoFormDataFieldArray, callback: extern fn(*const PlamoFormDataField)) {
    unsafe {
        (*plamo_form_data_field_array).iter().for_each(|value| callback(value));
    }
}

#[no_mangle]
pub extern fn plamo_form_data_field_array_get_at(plamo_form_data_field_array: *const PlamoFormDataFieldArray, index: usize) -> *const PlamoFormDataField {
    unsafe {
        match (*plamo_form_data_field_array).get(index) {
            Some(value) => value,
            None => std::ptr::null(),
        }
    }
}

#[no_mangle]
pub extern fn plamo_form_data_field_array_get_first(plamo_form_data_field_array: *const PlamoFormDataFieldArray) -> *const PlamoFormDataField {
    unsafe {
        match (*plamo_form_data_field_array).first() {
            Some(value) => value,
            None => std::ptr::null(),
        }
    }
}

#[no_mangle]
pub extern fn plamo_form_data_field_array_get_last(plamo_form_data_field_array: *const PlamoFormDataFieldArray) -> *const PlamoFormDataField {
    unsafe {
        match (*plamo_form_data_field_array).last() {
            Some(value) => value,
            None => std::ptr::null(),
        }
    }
}
