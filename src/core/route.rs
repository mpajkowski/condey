use http::method::Method;
use std::fmt::Display;

use super::handler::Handler;

pub struct Route {
    pub method: Method,
    pub path: String,
    pub handler: Box<dyn Handler + Send + Sync + 'static>,
}

impl Route {
    pub fn new<P, H>(method: Method, path: P, handler: H) -> Self
    where
        P: Display,
        H: Handler + Send + Sync + 'static,
    {
        Route {
            method,
            path: path.to_string(),
            handler: Box::new(handler),
        }
    }
}
