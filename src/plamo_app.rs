use plamo_middleware::PlamoMiddleware;
use std::ptr;

#[repr(C)]
pub struct PlamoApp {
    middlewares: Vec<PlamoMiddleware>,
}

#[no_mangle]
pub extern fn plamo_app_new() -> *mut PlamoApp {
    Box::into_raw(Box::new(PlamoApp {
        middlewares: Vec::new(),
    }))
}

#[no_mangle]
pub extern fn plamo_app_destroy(plamo_app: &mut *mut PlamoApp) {
    if !plamo_app.is_null() {
        unsafe {
           Box::from_raw(*plamo_app);
        }
        *plamo_app = ptr::null_mut();
    }
}
