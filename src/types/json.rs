use crate::{http::header, FromBody, Interceptor, Request, Responder, Response};

use hyper::{http::response::Builder, StatusCode};
use serde::{de::DeserializeOwned, Serialize};
use serde_json::error::Category;

use std::ops::{Deref, DerefMut};

pub struct Json<T>(pub T);

impl<T> From<T> for Json<T> {
    fn from(json: T) -> Self {
        Json(json)
    }
}

impl<T> Json<T> {
    pub fn into_inner(self) -> T {
        self.0
    }
}

impl<T> Deref for Json<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> DerefMut for Json<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[async_trait::async_trait]
impl<T: Serialize + Send + Sync> Responder for Json<T> {
    async fn respond_to(self, _: &Request) -> Response {
        Builder::new()
            .status(StatusCode::OK)
            .header(header::CONTENT_TYPE, "application/json")
            .body(serde_json::to_vec(&self.0).unwrap().into())
            .unwrap()
    }
}

#[async_trait::async_trait]
impl<'r, T: DeserializeOwned> FromBody<'r> for Json<T> {
    type Error = serde_json::Error;

    async fn from_body(_req: &'r Request, body: &'r [u8]) -> Result<Self, Self::Error> {
        let json = serde_json::from_slice(body)?;

        Ok(Json(json))
    }

    fn default_interceptor() -> Box<dyn Interceptor> {
        Box::new(JsonInterceptor)
    }
}

#[derive(Debug, Clone)]
pub struct JsonInterceptor;

#[async_trait::async_trait]
impl Interceptor for JsonInterceptor {
    async fn intercept(&self, _req: Request, body: Vec<u8>, err: anyhow::Error) -> Response {
        let err = err.downcast_ref::<serde_json::Error>().unwrap();

        let resp = serde_json::json!({
            "original_request": String::from_utf8_lossy(&*body).to_string(),
            "error_class": match err.classify() {
                Category::Io => "IO",
                Category::Syntax => "SYNTAX",
                Category::Data => "DATA",
                Category::Eof => "EOF",
            },
            "line": err.line(),
            "column": err.column(),
        });

        Builder::new()
            .status(if matches!(err.classify(), Category::Data) {
                StatusCode::UNPROCESSABLE_ENTITY
            } else {
                StatusCode::BAD_REQUEST
            })
            .header(header::CONTENT_TYPE, "application/json")
            .body(serde_json::to_vec(&resp).unwrap().into())
            .unwrap()
    }
}
