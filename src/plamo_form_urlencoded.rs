use crate::plamo_request::PlamoRequest;
use crate::plamo_string::PlamoString;
use crate::plamo_string_array::PlamoStringArray;
use std::collections::BTreeMap;
use std::ffi::CStr;
use std::os::raw::c_char;

pub type PlamoFormUrlencoded = BTreeMap<PlamoString, PlamoStringArray>;

#[no_mangle]
pub extern fn plamo_form_urlencoded_new(plamo_request: *const PlamoRequest) -> *mut PlamoFormUrlencoded {
    let headers = unsafe { (*(*plamo_request).header).get_mut(&PlamoString::new("content-type").unwrap()) };
    match headers {
        Some(headers) => {
            match headers.first() {
                Some(header) => {
                    if header == &PlamoString::new("application/x-www-form-urlencoded").unwrap() {
                        let mut plamo_form_urlencoded = PlamoFormUrlencoded::new();

                        url::form_urlencoded::parse(unsafe { &*(*plamo_request).body }).into_owned().for_each(|(key, value)| {
                            match plamo_form_urlencoded.get_mut(&PlamoString::new(key.clone()).unwrap()) {
                                Some(values) => {
                                    values.push(PlamoString::new(value).unwrap());
                                },
                                None => {
                                    plamo_form_urlencoded.insert(PlamoString::new(key).unwrap(), vec![PlamoString::new(value).unwrap()]);
                                }
                            }
                        });

                        Box::into_raw(Box::new(plamo_form_urlencoded))
                    } else {
                        std::ptr::null_mut()
                    }
                },
                None => std::ptr::null_mut()
            }
        },
        None => std::ptr::null_mut()
    }
}

#[no_mangle]
pub extern fn plamo_form_urlencoded_destroy(plamo_form_urlencoded: *mut PlamoFormUrlencoded) {
    unsafe {
       drop(Box::from_raw(plamo_form_urlencoded));
    }
}

#[no_mangle]
pub extern fn plamo_form_urlencoded_for_each(plamo_form_urlencoded: *const PlamoFormUrlencoded, callback: extern fn(*const c_char, *const c_char)) {
    unsafe {
        (*plamo_form_urlencoded).iter().for_each(|(key, values)| {
            values.iter().for_each(|value| callback(key.as_ptr(), value.as_ptr()));
        });
    }
}

#[no_mangle]
pub extern fn plamo_form_urlencoded_get(plamo_form_urlencoded: *const PlamoFormUrlencoded, key: *const c_char) -> *const PlamoStringArray {
    unsafe {
        match (*plamo_form_urlencoded).get(CStr::from_ptr(key)) {
            Some(values) => values,
            None => std::ptr::null(),
        }
    }
}
