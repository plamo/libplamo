use std::ffi::CString;
use std::os::raw::c_char;

#[repr(C)]
pub struct PlamoHttpQueries {
    inner: Vec<CString>,
}

#[no_mangle]
pub extern fn plamo_http_queries_for_each(plamo_http_queries: *const PlamoHttpQueries, callback: extern "C" fn(*const c_char)) {
    unsafe {
        (*plamo_http_queries).inner.iter().for_each(|header| callback(header.as_ptr()));
    }
}
