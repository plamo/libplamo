use std::os::raw::c_char;

#[repr(C)]
pub struct PlamoHttpHeaders(Vec<*const c_char>);
