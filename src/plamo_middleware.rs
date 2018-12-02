use plamo_request::PlamoRequest;
use plamo_response::PlamoResponse;
use std::os::raw::c_void;

#[repr(C)]
pub struct PlamoMiddleware {
    pub body: *const c_void,
    pub callback: extern "C" fn(*const c_void, *const PlamoRequest, *mut PlamoResponse) -> bool,
}

#[no_mangle]
pub extern fn plamo_middleware_new(body: *const c_void, callback: extern "C" fn(*const c_void, *const PlamoRequest, *mut PlamoResponse) -> bool) -> *mut PlamoMiddleware {
    Box::into_raw(Box::new(PlamoMiddleware {
        body: body,
        callback: callback,
    }))
}

#[no_mangle]
pub extern fn plamo_middleware_destroy(plamo_middleware: *mut PlamoMiddleware) {
    unsafe {
       drop(Box::from_raw(plamo_middleware));
    }
}
