use plamo_request::PlamoRequest;
use plamo_response::PlamoResponse;
use std::os::raw::c_void;
use std::ptr;

#[repr(C)]
pub struct PlamoMiddleware {
    pub(crate) body: *const c_void,
    pub(crate) callback: *const extern fn(*const c_void, *const PlamoRequest, *mut PlamoResponse) -> bool,
}

#[no_mangle]
pub extern fn plamo_middleware_new(body: *const c_void, callback: *const extern fn(*const c_void, *const PlamoRequest, *mut PlamoResponse) -> bool) -> *mut PlamoMiddleware {
    Box::into_raw(Box::new(PlamoMiddleware {
        body: body,
        callback: callback,
    }))
}

#[no_mangle]
pub extern fn plamo_middleware_destroy(plamo_middleware: &mut *mut PlamoMiddleware) {
    if !plamo_middleware.is_null() {
        unsafe {
           Box::from_raw(*plamo_middleware);
        }
        *plamo_middleware = ptr::null_mut();
    }
}
