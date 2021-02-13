use crate::{Body, FromBody, Request, Responder, Response};

use futures::TryStreamExt;
use hyper::{header, http::response::Builder, StatusCode};
use serde::{de::DeserializeOwned, Serialize};
use thiserror::Error;

use std::ops::{Deref, DerefMut};

pub struct Form<T>(T);

impl<T> From<T> for Form<T> {
    fn from(form: T) -> Self {
        Form(form)
    }
}

impl<T> Form<T> {
    pub fn into_inner(self) -> T {
        self.0
    }
}

impl<T> Deref for Form<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> DerefMut for Form<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[derive(Debug, Error)]
pub enum ParseFormError {
    #[error("IO error occurred while parsing a form: `{0}`")]
    Io(#[from] hyper::Error),

    #[error("Deserialization error occurred while parsing a form: `{0}`")]
    Deserialize(#[from] serde_urlencoded::de::Error),
}

#[async_trait::async_trait]
impl<T: Serialize + Send + Sync> Responder for Form<T> {
    async fn respond_to(self, _: &Request) -> Response {
        Builder::new()
            .status(StatusCode::OK)
            .header(header::CONTENT_TYPE, "application/x-www-form-urlencoded")
            .body(serde_urlencoded::to_string(&self.0).unwrap().into())
            .unwrap()
    }
}

#[async_trait::async_trait]
impl<'r, T: DeserializeOwned> FromBody<'r> for Form<T> {
    type Error = ParseFormError;

    async fn from_body(_req: &'r Request, body: &'r mut Body) -> Result<Self, Self::Error>
    where
        Self: Sized,
    {
        let body: Vec<u8> = body
            .map_ok(|chunk| chunk.into_iter().collect::<Vec<u8>>())
            .try_concat()
            .await?;

        let form = serde_urlencoded::from_bytes(&*body)?;

        Ok(Form(form))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use serde::Deserialize;

    #[tokio::test]
    async fn extract_query() {
        #[derive(Debug, Deserialize)]
        struct Foo {
            bread: String,
            cheese: String,
        }

        let mut body = Body::from("bread=baguette&cheese=comt%C3%A9");
        let request = Request::new(Body::empty());

        let extracted = Form::<Foo>::from_body(&request, &mut body)
            .await
            .unwrap()
            .into_inner();

        println!("{:?}", extracted);
    }

    #[tokio::test]
    async fn allow_nones() {
        #[derive(Debug, Deserialize)]
        struct Foo {
            bread: Option<String>,
            cheese: Option<String>,
        }

        let mut body = Body::from("");
        let request = Request::new(Body::empty());

        let extracted = Form::<Foo>::from_body(&request, &mut body)
            .await
            .unwrap()
            .into_inner();

        println!("{:?}", extracted);
    }
}
