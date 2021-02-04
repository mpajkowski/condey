use crate::http::request::Request as HttpRequest;
use hyper::Body;

pub type Request = HttpRequest<Body>;
