use super::request::Request;
use crate::Interceptor;

use anyhow::Result;

pub trait ExtractClass: Send + Sync + 'static {}

#[async_trait::async_trait]
pub trait Extract<'r, T: ExtractClass> {
    async fn extract(request: &'r Request, _: &'r [u8]) -> Result<Self>
    where
        Self: Sized + 'r;

    fn default_interceptor() -> Box<dyn Interceptor>;

    const TAKES_BODY: bool;
}
