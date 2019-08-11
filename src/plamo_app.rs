use crate::plamo_middleware::PlamoMiddleware;
use crate::plamo_request::PlamoRequest;
use crate::plamo_response::{PlamoResponse, plamo_response_new};

#[repr(C)]
pub struct PlamoApp {
    middlewares: Vec<*const PlamoMiddleware>,
}

#[no_mangle]
pub extern fn plamo_app_new() -> *mut PlamoApp {
    Box::into_raw(Box::new(PlamoApp {
        middlewares: Vec::new(),
    }))
}

#[no_mangle]
pub extern fn plamo_app_destroy(plamo_app: *mut PlamoApp) {
    unsafe {
        drop(Box::from_raw(plamo_app));
    }
}

#[no_mangle]
pub extern fn plamo_app_add_middleware(plamo_app: *mut PlamoApp, plamo_middleware: *const PlamoMiddleware) {
    unsafe {
        (*plamo_app).middlewares.push(plamo_middleware);
    }
}

#[no_mangle]
pub extern fn plamo_app_execute(plamo_app: *const PlamoApp, plamo_request: *const PlamoRequest) -> *mut PlamoResponse {
    let plamo_response = plamo_response_new();

    unsafe {
        for middleware in &(*plamo_app).middlewares {
            ((**middleware).callback)((**middleware).config, plamo_request, plamo_response);
            if (*plamo_response).status_code >= 300 {
                break;
            }
        }
    }

    plamo_response
}
