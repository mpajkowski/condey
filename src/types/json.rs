use crate::{http::header, FromBody, Request, Responder, Response};

use futures::TryStreamExt;
use hyper::{http::response::Builder, Body, StatusCode};
use serde::{de::DeserializeOwned, Serialize};

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
impl<T: DeserializeOwned> FromBody for Json<T> {
    async fn from_body(_req: &Request, body: &mut Body) -> anyhow::Result<Self> {
        let body: Vec<u8> = body
            .map_ok(|chunk| chunk.into_iter().collect::<Vec<u8>>())
            .try_concat()
            .await?;

        let json = serde_json::from_slice(&*body)?;

        Ok(Json(json))
    }
}
