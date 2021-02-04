use anyhow::Result;
use condey::Route;
use condey::{http::Method, Json};
use condey::{Condey, Fn0, Fn1, Path};
use serde::Serialize;

async fn root_callback() -> String {
    "Hello!".into()
}

async fn id_callback(Path((p1,)): Path<(String,)>) -> String {
    format!("extracted: {}", p1)
}

async fn assignments_callback(Path((p1, p2)): Path<(String, String)>) -> String {
    format!("extracted: {} and {}", p1, p2)
}

#[derive(Debug, Serialize)]
pub struct SendMeJson {
    foo: String,
    bar: u32,
}

async fn send_me_json() -> Json<SendMeJson> {
    Json(SendMeJson {
        foo: "Foo".into(),
        bar: 42,
    })
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
        Route::new(Method::GET, "/json_heaven", Fn0::from(send_me_json)),
    ];

    Condey::init()
        .mount("/api/test", routes)
        .listen_at("127.0.0.1:3000")
        .await?;

    Ok(())
}
