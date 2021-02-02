use std::marker::PhantomData;

use http::Method;
use http::Response as HttpResponse;
use hyper::Body;

pub struct Response<T>(pub HttpResponse<Body>, pub PhantomData<T>);

impl<T> Response<T> {
    pub fn new(r: HttpResponse<Body>) -> Self {
        Self(r, PhantomData)
    }
}

impl<T> Into<HttpResponse<Body>> for Response<T> {
    fn into(self) -> HttpResponse<Body> {
        self.0
    }
}

#[derive(Debug)]
pub struct BodyDescription {
    method: Method,
    path: String,
}

pub trait IntoBody {
    fn into_body(self) -> Body;
}

impl<T: Into<Body>> IntoBody for T {
    fn into_body(self) -> Body {
        T::into(self)
    }
}
