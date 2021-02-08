use super::request::Request;
use crate::Body;

use anyhow::Result;

pub trait ExtractClass: Send + Sync + 'static {}

#[async_trait::async_trait]
pub trait Extract<'r, T: ExtractClass> {
    async fn extract(request: &'r Request, _: &'r mut Body) -> Result<Self>
    where
        Self: Sized + 'r;

    fn takes_body() -> bool;
}

#[async_trait::async_trait]
impl<'r, T: ExtractClass> Extract<'r, T> for &'r Request {
    async fn extract(request: &'r Request, _: &'r mut Body) -> Result<Self> {
        Ok(request)
    }

    fn takes_body() -> bool {
        false
    }
}
