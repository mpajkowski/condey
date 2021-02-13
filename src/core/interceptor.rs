use dyn_clone::DynClone;

use crate::{Request, Responder, Response};

#[async_trait::async_trait]
pub trait Interceptor: DynClone + Send + Sync {
    async fn respond_to(&self, req: &Request) -> Response;
}

#[async_trait::async_trait]
impl<T: Responder + Clone> Interceptor for T {
    async fn respond_to(&self, req: &Request) -> Response {
        let contents = self.clone();

        Responder::respond_to(contents, req).await
    }
}
