use futures::TryStreamExt;
use serde::{de::DeserializeOwned, Serialize};

use crate::{Body, Extract, Request, Responder, Response};
use hyper::{header, http::response::Builder, StatusCode};

pub struct Query<T>(T);

impl<T> From<T> for Query<T> {
    fn from(query: T) -> Self {
        Query(query)
    }
}

impl<T> Query<T> {
    pub fn into_inner(self) -> T {
        self.0
    }
}

#[async_trait::async_trait]
impl<'r, T: DeserializeOwned> Extract<'r> for Query<T> {
    async fn extract(req: &'r Request, _body: &mut Body) -> anyhow::Result<Self>
    where
        Self: Sized,
    {
        let query = req.uri().query().unwrap_or_default();
        let query = serde_urlencoded::from_str(&*query)?;

        Ok(Query(query))
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

        let mut request = Request::new(Body::empty());
        let uri = request.uri_mut();
        *uri = "/test?bread=baguette&cheese=comte".parse().unwrap();

        let extracted: Foo = Query::extract(&request, &mut Body::empty())
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

        let mut request = Request::new(Body::empty());
        let uri = request.uri_mut();
        *uri = "/test".parse().unwrap();

        let extracted: Foo = Query::extract(&request, &mut Body::empty())
            .await
            .unwrap()
            .into_inner();

        println!("{:?}", extracted);
    }
}
