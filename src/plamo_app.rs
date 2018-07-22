use plamo_middleware::PlamoMiddleware;
use plamo_request::PlamoRequest;
use plamo_response::{PlamoResponse, plamo_response_new};
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

#[no_mangle]
pub extern fn plamo_app_execute(plamo_app: *const PlamoApp, plamo_request: *const PlamoRequest) -> *mut PlamoResponse {
    let plamo_response = plamo_response_new();

    unsafe {
        for middleware in &(*plamo_app).middlewares {
            if !(*middleware.callback)(middleware.body, plamo_request, plamo_response) {
                break;
            }
        }
    }

    plamo_response
}
