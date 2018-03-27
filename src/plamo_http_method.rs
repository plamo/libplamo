#[repr(C)]
#[derive(Copy, Clone)]
pub enum PlamoHttpMethod {
    Get,
    Head,
    Post,
    Put,
    Delete,
    Connect,
    Options,
    Trace,
}
