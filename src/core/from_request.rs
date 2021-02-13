use std::convert::Infallible;

use hyper::StatusCode;

use crate::{Interceptor, Request};

use super::extract::{Extract, ExtractClass};

#[async_trait::async_trait]
pub trait FromRequest<'r> {
    type Error: Into<anyhow::Error> + 'static;

    async fn from_request(request: &'r Request) -> Result<Self, Self::Error>
    where
        Self: Sized + 'r;

    fn default_interceptor() -> Box<dyn Interceptor> {
        Box::new(StatusCode::NOT_FOUND)
    }
}

#[async_trait::async_trait]
impl<'r, T, E: Into<anyhow::Error> + 'static> Extract<'r, ExtractRequest> for T
where
    T: FromRequest<'r, Error = E>,
    Self: 'r,
{
    #[inline(always)]
    async fn extract(request: &'r Request, _: &'r mut hyper::Body) -> anyhow::Result<Self> {
        T::from_request(request).await.map_err(|err| err.into())
    }

    const TAKES_BODY: bool = false;

    fn default_interceptor() -> Box<dyn Interceptor> {
        T::default_interceptor()
    }
}

pub struct ExtractRequest;
impl ExtractClass for ExtractRequest {}

#[async_trait::async_trait]
impl<'r> FromRequest<'r> for &'r Request {
    type Error = Infallible;

    async fn from_request(request: &'r Request) -> Result<Self, Self::Error> {
        Ok(request)
    }
}
