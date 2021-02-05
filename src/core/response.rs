use crate::http::{response::Builder, Response as HttpResponse};
use hyper::{Body, StatusCode};

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
