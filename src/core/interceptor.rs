use dyn_clone::DynClone;
use hyper::StatusCode;

use crate::{Request, Responder, Response};

#[async_trait::async_trait]
pub trait Interceptor: DynClone + Send + Sync {
    async fn intercept(&self, req: Request, body: Vec<u8>, err: anyhow::Error) -> Response;
}

#[async_trait::async_trait]
impl Interceptor for StatusCode {
    async fn intercept(&self, req: Request, _body: Vec<u8>, _err: anyhow::Error) -> Response {
        self.respond_to(&req).await
    }
}
