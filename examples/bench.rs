use anyhow::Result;
use condey::{http::Method, Condey, Route};

async fn hello() -> String {
    "Hello, World!".into()
}

async fn another() -> String {
    "Aunother route".into()
}

#[tokio::main]
async fn main() -> Result<()> {
    let routes: Vec<Route> = vec![
        Route::builder()
            .method(Method::GET)
            .path("/hello")
            .with_handler_fn(hello),
        Route::builder()
            .method(Method::GET)
            .path("/another")
            .with_handler_fn(another),
    ];

    Condey::init()
        .mount("", routes)
        .listen_at("127.0.0.1:3000")
        .await?;

    Ok(())
}
