use http::method::Method;
use std::fmt::Display;

#[derive(Debug)]
pub struct Handler {
    pub method: Method,
    pub path: String,
    pub cb: String,
}

impl Handler {
    pub fn new<P: Display, C: Display>(method: Method, path: P, cb: C) -> Self {
        Handler {
            method,
            path: path.to_string(),
            cb: cb.to_string(),
        }
    }
}
