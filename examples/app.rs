use std::marker::PhantomData;

use anyhow::Result;
use condey::{Condey, Extract, Fn0, Fn1, Handler, Path, Request};
use condey::{Response, Route};
use http::method::Method;
use hyper::Body;

async fn root_callback() -> Response<Vec<u8>> {
    Response::new(hyper::Response::new("Hello!".into()))
}

async fn id_callback(Path((p1,)): Path<(String,)>) -> Response<Body> {
    Response::new(hyper::Response::new(format!("extracted: {}", p1).into()))
}

async fn assignments_callback(Path((p1, p2)): Path<(String, String)>) -> Response<Body> {
    Response::new(hyper::Response::new(
        format!("extracted: {} and {}", p1, p2).into(),
    ))
}

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt().try_init().unwrap();

    let routes: Vec<Route> = vec![
        Route::new(Method::GET, "/employees", Fn0::from(root_callback)),
        Route::new(Method::GET, "/employees/:id", Fn1::from(id_callback)),
        Route::new(
            Method::GET,
            "/employees/:id/assignments",
            Fn1::from(id_callback),
        ),
        Route::new(
            Method::GET,
            "/employees/:id/assignments/:assignment_id",
            Fn1::from(assignments_callback),
        ),
    ];

    Condey::init()
        .mount("/api/test", routes)
        .listen_at("127.0.0.1:3000")
        .await?;

    Ok(())
}
