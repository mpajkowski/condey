use crate::{http::header, FromBody, Request, Responder, Response};

use futures::TryStreamExt;
use hyper::{http::response::Builder, Body, StatusCode};
use serde::{de::DeserializeOwned, Serialize};
use thiserror::Error;

use std::ops::{Deref, DerefMut};

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

impl<T> Deref for Json<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> DerefMut for Json<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[derive(Debug, Error)]
pub enum ParseJsonError {
    #[error("IO error occurred while parsing a form: `{0}`")]
    Io(#[from] hyper::Error),

    #[error("Deserialization error occurred while parsing a json body: `{0}`")]
    Deserialize(#[from] serde_json::Error),
}

#[async_trait::async_trait]
impl<T: Serialize + Send + Sync> Responder for Json<T> {
    async fn respond_to(self, _: &Request) -> Response {
        Builder::new()
            .status(StatusCode::OK)
            .header(header::CONTENT_TYPE, "application/json")
            .body(serde_json::to_vec(&self.0).unwrap().into())
            .unwrap()
    }
}

#[async_trait::async_trait]
impl<'r, T: DeserializeOwned> FromBody<'r> for Json<T> {
    type Error = ParseJsonError;

    async fn from_body(_req: &'r Request, body: &'r mut Body) -> Result<Self, Self::Error> {
        let body: Vec<u8> = body
            .map_ok(|chunk| chunk.into_iter().collect::<Vec<u8>>())
            .try_concat()
            .await?;

        let json = serde_json::from_slice(&*body)?;

        Ok(Json(json))
    }
}
