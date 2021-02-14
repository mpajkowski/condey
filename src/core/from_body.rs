use super::extract::{Extract, ExtractClass};
use crate::{Interceptor, Request};

use hyper::StatusCode;

#[async_trait::async_trait]
pub trait FromBody<'r> {
    type Error: Into<anyhow::Error> + 'static;

    async fn from_body(request: &'r Request, body: &'r [u8]) -> Result<Self, Self::Error>
    where
        Self: Sized + 'r;

    fn default_interceptor() -> Box<dyn Interceptor> {
        Box::new(StatusCode::BAD_REQUEST)
    }
}

#[async_trait::async_trait]
impl<'r, T, E: Into<anyhow::Error> + 'static> Extract<'r, ExtractBody> for T
where
    T: FromBody<'r, Error = E>,
    Self: 'r,
{
    #[inline(always)]
    async fn extract(request: &'r Request, body: &'r [u8]) -> anyhow::Result<Self> {
        T::from_body(request, body).await.map_err(|err| err.into())
    }

    const TAKES_BODY: bool = true;

    fn default_interceptor() -> Box<dyn Interceptor> {
        T::default_interceptor()
    }
}

pub struct ExtractBody;
impl ExtractClass for ExtractBody {}
