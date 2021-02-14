use crate::http::{response::Builder, Response as HttpResponse};
use crate::Request;

use hyper::{Body, StatusCode};

#[async_trait::async_trait]
pub trait Responder: Send + Sync {
    async fn respond_to(self, req: &Request) -> Response;
}

pub type Response = HttpResponse<Body>;

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
impl Responder for Response {
    async fn respond_to(self, _: &Request) -> Response {
        self
    }
}

#[async_trait::async_trait]
impl Responder for StatusCode {
    async fn respond_to(self, _: &Request) -> Response {
        Builder::new().status(self).body(Body::default()).unwrap()
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

#[async_trait::async_trait]
impl<T: Responder + Send> Responder for Option<T> {
    async fn respond_to(self, req: &Request) -> Response {
        match self {
            Some(r) => r.respond_to(req).await,
            None => StatusCode::NOT_FOUND.respond_to(req).await,
        }
    }
}

#[async_trait::async_trait]
impl<T, E> Responder for Result<T, E>
where
    T: Responder + Send,
    E: Responder + Send,
{
    async fn respond_to(self, req: &Request) -> Response {
        match self {
            Ok(ok) => ok.respond_to(req).await,
            Err(err) => err.respond_to(req).await,
        }
    }
}
