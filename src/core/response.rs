use crate::http::{response::Builder, Response as HttpResponse};
use hyper::{Body, StatusCode};
use serde::Serialize;

use crate::Request;

#[async_trait::async_trait]
pub trait Responder {
    async fn respond_to(self, req: &Request) -> Response;
}

pub type Response = HttpResponse<Body>;

#[async_trait::async_trait]
impl Responder for Response {
    async fn respond_to(self, _: &Request) -> Response {
        self
    }
}

#[async_trait::async_trait]
impl Responder for String {
    async fn respond_to(self, _: &Request) -> Response {
        Builder::new()
            .status(StatusCode::OK)
            .header(crate::http::header::CONTENT_TYPE, "text/plain")
            .body(self.into())
            .unwrap()
    }
}

#[async_trait::async_trait]
impl Responder for Vec<u8> {
    async fn respond_to(self, _: &Request) -> Response {
        Builder::new()
            .status(StatusCode::OK)
            .header(
                crate::http::header::CONTENT_TYPE,
                "application/octet-stream",
            )
            .body(self.into())
            .unwrap()
    }
}

pub struct Json<T>(pub T);

#[async_trait::async_trait]
impl<T: Serialize + Send> Responder for Json<T> {
    async fn respond_to(self, _: &Request) -> Response {
        Builder::new()
            .status(StatusCode::OK)
            .header(crate::http::header::CONTENT_TYPE, "application/json")
            .body(serde_json::to_vec(&self.0).unwrap().into())
            .unwrap()
    }
}
