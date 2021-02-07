use futures::TryStreamExt;
use serde::{de::DeserializeOwned, Serialize};

use crate::{Body, Extract, Request, Responder, Response};
use hyper::{header, http::response::Builder, StatusCode};

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

#[async_trait::async_trait]
impl<T: Serialize + Send> Responder for Form<T> {
    async fn respond_to(self, _: &Request) -> Response {
        Builder::new()
            .status(StatusCode::OK)
            .header(header::CONTENT_TYPE, "application/x-www-form-urlencoded")
            .body(serde_urlencoded::to_string(&self.0).unwrap().into())
            .unwrap()
    }
}

#[async_trait::async_trait]
impl<'r, T: DeserializeOwned> Extract<'r> for Form<T> {
    async fn extract(_req: &'r Request, body: &mut Body) -> anyhow::Result<Self>
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

    fn takes_body() -> bool {
        true
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

        let extracted: Foo = Form::extract(&request, &mut body)
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

        let extracted: Foo = Form::extract(&request, &mut body)
            .await
            .unwrap()
            .into_inner();

        println!("{:?}", extracted);
    }
}
