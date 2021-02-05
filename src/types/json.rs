use futures::stream::TryStreamExt;
use hyper::{http::response::Builder, StatusCode};
use serde::{de::DeserializeOwned, Serialize};

use crate::{http::header, Extract, Request, Responder, Response};

pub struct Json<T>(pub T);

impl<T> From<T> for Json<T> {
    fn from(json: T) -> Self {
        Json(json)
    }
}

impl<T> Json<T> {
    pub fn into_inner(self) -> T {
        self.0
    }
}

#[async_trait::async_trait]
impl<T: Serialize + Send> Responder for Json<T> {
    async fn respond_to(self, _: &Request) -> Response {
        Builder::new()
            .status(StatusCode::OK)
            .header(header::CONTENT_TYPE, "application/json")
            .body(serde_json::to_vec(&self.0).unwrap().into())
            .unwrap()
    }
}

#[async_trait::async_trait]
impl<'r, T: DeserializeOwned> Extract<'r> for Json<T> {
    async fn extract(request: &'r mut Request) -> anyhow::Result<Self>
    where
        Self: Sized,
    {
        let body: Vec<u8> = request
            .body_mut()
            .map_ok(|chunk| chunk.into_iter().collect::<Vec<u8>>())
            .try_concat()
            .await?;

        let json = serde_json::from_slice(&*body)?;

        Ok(Json(json))
    }
}
