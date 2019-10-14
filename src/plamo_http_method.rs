use std::ffi::CString;
use std::os::raw::c_char;
use std::ptr;

pub type PlamoDefinedHttpMethod = usize;

pub const PLAMO_HTTP_METHOD_GET: PlamoDefinedHttpMethod = usize::max_value();
pub const PLAMO_HTTP_METHOD_POST: PlamoDefinedHttpMethod = usize::max_value() - 1;
pub const PLAMO_HTTP_METHOD_PUT: PlamoDefinedHttpMethod = usize::max_value() - 2;
pub const PLAMO_HTTP_METHOD_DELETE: PlamoDefinedHttpMethod = usize::max_value() - 3;
pub const PLAMO_HTTP_METHOD_HEAD: PlamoDefinedHttpMethod = usize::max_value() - 4;
pub const PLAMO_HTTP_METHOD_CONNECT: PlamoDefinedHttpMethod = usize::max_value() - 5;
pub const PLAMO_HTTP_METHOD_OPTIONS: PlamoDefinedHttpMethod = usize::max_value() - 6;
pub const PLAMO_HTTP_METHOD_TRACE: PlamoDefinedHttpMethod = usize::max_value() - 7;
pub const PLAMO_HTTP_METHOD_PATCH: PlamoDefinedHttpMethod = usize::max_value() - 8;

#[repr(C)]
pub union PlamoHttpMethod {
    pub defined_http_method: PlamoDefinedHttpMethod,
    pub undefined_http_method: *mut c_char,
}

#[no_mangle]
pub extern fn plamo_http_method_new(method: usize) -> PlamoHttpMethod {
    if method >= PLAMO_HTTP_METHOD_PATCH {
        PlamoHttpMethod {
            defined_http_method: method
        }
    } else {
        if method == 0 {
            PlamoHttpMethod {
                undefined_http_method: ptr::null_mut()
            }
        } else {
            unsafe {
                PlamoHttpMethod {
                    undefined_http_method: CString::from_raw(method as *mut c_char).into_raw()
                }
            }
        }
    }
}

#[no_mangle]
pub extern fn plamo_http_method_destroy(plamo_http_method: *mut PlamoHttpMethod) {
    unsafe {
        if (*plamo_http_method).defined_http_method < PLAMO_HTTP_METHOD_PATCH && (*plamo_http_method).defined_http_method != 0 {
            CString::from_raw((*plamo_http_method).undefined_http_method);
            (*plamo_http_method).undefined_http_method = ptr::null_mut();
        } 
    }
}
