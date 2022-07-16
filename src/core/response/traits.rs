use crate::{Request, Response};

use super::Status;

#[async_trait::async_trait]
pub trait Responder: Send + Sync {
    async fn respond_to(self, req: &Request) -> Response;

    fn status<const S: u16>(self) -> Status<Self, S>
    where
        Self: Sized,
    {
        Status::new(self)
    }
}
