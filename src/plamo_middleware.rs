use crate::plamo_request::PlamoRequest;
use crate::plamo_response::PlamoResponse;
use std::os::raw::c_void;

#[repr(C)]
pub struct PlamoMiddleware {
    pub config: *const c_void,
    pub callback: extern "C" fn(*const c_void, *const PlamoRequest, *mut PlamoResponse),
}

#[no_mangle]
pub extern fn plamo_middleware_new(config: *const c_void, callback: extern "C" fn(*const c_void, *const PlamoRequest, *mut PlamoResponse)) -> *mut PlamoMiddleware {
    Box::into_raw(Box::new(PlamoMiddleware {
        config,
        callback,
    }))
}

#[no_mangle]
pub extern fn plamo_middleware_destroy(plamo_middleware: *mut PlamoMiddleware) {
    unsafe {
       drop(Box::from_raw(plamo_middleware));
    }
}
