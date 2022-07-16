use hyper::StatusCode;

use crate::{Request, Responder, Response};

pub struct Status<R, const S: u16>(R);

impl<R, const S: u16> Status<R, S> {
    pub fn new(responder: R) -> Self {
        Status(responder)
    }
}

#[async_trait::async_trait]
impl<R: Responder, const S: u16> Responder for Status<R, S> {
    async fn respond_to(self, req: &Request) -> Response {
        let mut response = self.0.respond_to(req).await;
        let status = response.status_mut();
        *status = StatusCode::from_u16(S).unwrap();
        response
    }
}
