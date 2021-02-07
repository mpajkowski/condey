use crate::http::request::Request as HttpRequest;
use crate::Body;

pub type Request = HttpRequest<Body>;
