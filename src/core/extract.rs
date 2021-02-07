use super::request::Request;
use crate::Body;
use anyhow::Result;

#[async_trait::async_trait]
pub trait Extract<'r> {
    async fn extract(request: &'r Request, _: &mut Body) -> Result<Self>
    where
        Self: Sized;

    fn takes_body() -> bool {
        false
    }
}

#[async_trait::async_trait]
impl<'r> Extract<'r> for &'r Request {
    async fn extract(request: &'r Request, _: &mut Body) -> Result<Self> {
        Ok(request)
    }
}
