use crate::Request;

use anyhow::Result;

use super::extract::{Extract, ExtractClass};

#[async_trait::async_trait]
pub trait FromRequest<'r> {
    async fn from_request(request: &'r Request) -> Result<Self>
    where
        Self: Sized;
}

#[async_trait::async_trait]
impl<'r, T> Extract<'r, ExtractRequest> for T
where
    T: FromRequest<'r>,
{
    #[inline(always)]
    async fn extract(request: &'r Request, _: &'r mut hyper::Body) -> anyhow::Result<Self> {
        T::from_request(request).await
    }

    #[inline(always)]
    fn takes_body() -> bool {
        false
    }
}

pub struct ExtractRequest;
impl ExtractClass for ExtractRequest {}
