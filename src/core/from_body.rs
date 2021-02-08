use crate::Request;

use anyhow::Result;
use hyper::Body;

use super::extract::{Extract, ExtractClass};

#[async_trait::async_trait]
pub trait FromBody {
    async fn from_body(request: &Request, body: &mut Body) -> Result<Self>
    where
        Self: Sized;
}

#[async_trait::async_trait]
impl<'r, T> Extract<'r, ExtractBody> for T
where
    T: FromBody,
{
    #[inline(always)]
    async fn extract(request: &'r Request, body: &'r mut Body) -> anyhow::Result<Self> {
        T::from_body(request, body).await
    }

    #[inline(always)]
    fn takes_body() -> bool {
        true
    }
}

pub struct ExtractBody;
impl ExtractClass for ExtractBody {}
