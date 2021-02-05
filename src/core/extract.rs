use super::request::Request;
use anyhow::Result;

#[async_trait::async_trait]
pub trait Extract<'r> {
    async fn extract(request: &'r mut Request) -> Result<Self>
    where
        Self: Sized;
}

#[async_trait::async_trait]
impl<'r> Extract<'r> for &'r Request {
    async fn extract(request: &'r mut Request) -> Result<Self> {
        Ok(request)
    }
}
