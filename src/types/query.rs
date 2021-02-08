use crate::{Body, FromRequest, Request};

use serde::de::DeserializeOwned;

use std::ops::{Deref, DerefMut};

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

impl<T> Deref for Query<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> DerefMut for Query<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[async_trait::async_trait]
impl<'r, T: DeserializeOwned> FromRequest<'r> for Query<T> {
    async fn from_request(req: &'r Request) -> anyhow::Result<Self>
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

        let extracted = Query::<Foo>::from_request(&request)
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

        let extracted = Query::<Foo>::from_request(&request)
            .await
            .unwrap()
            .into_inner();

        println!("{:?}", extracted);
    }
}
