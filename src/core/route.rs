use super::handler::Handler;
use crate::{
    http::method::Method,
    openapi::generator::{OpenApiGenerator, OpenApiResponder},
    HandlerFn, OpenApiResponse,
};

use std::{fmt::Display, sync::Arc};

#[derive(Clone)]
pub struct Route {
    pub(crate) method: Method,
    pub(crate) path: String,
    pub(crate) description: Option<String>,
    pub(crate) handler: Arc<dyn Handler>,
    pub(crate) open_api_responses: Vec<OpenApiResponse>,
}

impl Route {
    pub fn new<H>(
        method: Method,
        path: String,
        description: Option<String>,
        handler: H,
        open_api_responses: Vec<OpenApiResponse>,
    ) -> Self
    where
        H: Handler,
    {
        Route {
            method,
            path,
            description,
            handler: Arc::new(handler),
            open_api_responses,
        }
    }

    pub fn builder() -> RouteBuilder {
        RouteBuilder::default()
    }
}

pub struct RouteBuilder {
    pub(crate) method: Option<Method>,
    pub(crate) path: Option<String>,
    pub(crate) description: Option<String>,
}

impl Default for RouteBuilder {
    fn default() -> RouteBuilder {
        RouteBuilder {
            method: None,
            path: None,
            description: None,
        }
    }
}

impl RouteBuilder {
    pub fn description<S: Display>(self, description: S) -> Self {
        RouteBuilder {
            description: Some(description.to_string()),
            ..Default::default()
        }
    }

    pub fn method(self, method: Method) -> Self {
        RouteBuilder {
            method: Some(method),
            ..Default::default()
        }
    }

    pub fn path<S: Display>(self, path: S) -> Self {
        RouteBuilder {
            method: self.method,
            path: Some(path.to_string()),
            ..Default::default()
        }
    }

    pub fn handler<H: Handler>(self, handler: H) -> Route {
        Route::new(
            self.method.unwrap(),
            self.path.unwrap(),
            self.description,
            handler,
            vec![],
        )
    }

    pub fn handler_fn<H, F, P, R>(self, handler_fn: H) -> Route
    where
        H: Into<HandlerFn<F, P, R>>,
        HandlerFn<F, P, R>: Handler,
    {
        Route::new(
            self.method.unwrap(),
            self.path.unwrap(),
            self.description,
            handler_fn.into(),
            vec![],
        )
    }

    pub fn handler_fn_and_openapi<H, F, P, R>(
        self,
        gen: &mut OpenApiGenerator,
        handler_fn: H,
    ) -> Route
    where
        R: OpenApiResponder,
        H: Into<HandlerFn<F, P, R>>,
        HandlerFn<F, P, R>: Handler,
    {
        Route::new(
            self.method.unwrap(),
            self.path.unwrap(),
            self.description,
            handler_fn.into(),
            R::open_api_responses(gen),
        )
    }
}
