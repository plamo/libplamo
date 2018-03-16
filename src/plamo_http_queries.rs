use std::os::raw::c_char;

#[repr(C)]
pub struct PlamoHttpQueries(Vec<*const c_char>);
