use crate::http::method::Method;
use std::{fmt::Display, marker::PhantomData};

use super::handler::Handler;

pub struct Route {
    pub(crate) method: Method,
    pub(crate) path: String,
    pub(crate) handler: Box<dyn Handler>,
}

impl Route {
    pub fn new<P, H>(method: Method, path: P, handler: H) -> Self
    where
        P: Display,
        H: Handler,
    {
        Route {
            method,
            path: path.to_string(),
            handler: Box::new(handler),
        }
    }

    pub fn builder() -> RouteBuilder<AddMethod> {
        RouteBuilder::default()
    }
}

pub trait RouteBuilderState {}
pub struct AddMethod;
impl RouteBuilderState for AddMethod {}

pub struct AddPath;
impl RouteBuilderState for AddPath {}

pub struct WithHandler;
impl RouteBuilderState for WithHandler {}

pub struct RouteBuilder<T: RouteBuilderState> {
    method: Option<Method>,
    path: Option<String>,
    state: PhantomData<T>,
}

impl<T: RouteBuilderState> Default for RouteBuilder<T> {
    fn default() -> RouteBuilder<T> {
        RouteBuilder {
            method: None,
            path: None,
            state: PhantomData,
        }
    }
}

impl RouteBuilder<AddMethod> {
    pub fn method(self, method: Method) -> RouteBuilder<AddPath> {
        RouteBuilder {
            method: Some(method),
            ..Default::default()
        }
    }
}

impl RouteBuilder<AddPath> {
    pub fn path<S: Display>(self, path: S) -> RouteBuilder<WithHandler> {
        RouteBuilder {
            method: self.method,
            path: Some(path.to_string()),
            ..Default::default()
        }
    }
}

impl RouteBuilder<WithHandler> {
    pub fn with_handler<H: Handler + Send + Sync + 'static>(self, handler: H) -> Route {
        Route::new(self.method.unwrap(), self.path.unwrap(), handler)
    }
}
