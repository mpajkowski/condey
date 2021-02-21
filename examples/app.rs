use condey::{
    http::Method,
    types::{Json, Path},
    Condey, Route,
};

use anyhow::Result;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use tracing::Level;

async fn root() -> String {
    "Hello!".into()
}

async fn employee_by_id(Path((p1,)): Path<(String,)>) -> String {
    format!("extracted: {}", p1)
}

async fn assignment_by_id(Path((p1, p2)): Path<(String, String)>) -> String {
    format!("extracted: {} and {}", p1, p2)
}

#[derive(Debug, Deserialize, Serialize, JsonSchema)]
pub struct Album {
    band: String,
    name: String,
    year: u32,
}

async fn crystal_logic() -> Json<Album> {
    Album {
        band: "Manilla Road".to_owned(),
        name: "Crystal Logic".to_owned(),
        year: 1983,
    }
    .into()
}

async fn thanks_for_album(album: Json<Album>) -> String {
    let Album { band, name, year } = album.into_inner();

    format!(r#"Thanks for "{}" by {} ({})!"#, name, band, year)
}

#[tokio::main]
async fn main() -> Result<()> {
    let subscriber = tracing_subscriber::FmtSubscriber::builder()
        .with_max_level(Level::DEBUG)
        .finish();

    tracing::subscriber::set_global_default(subscriber)?;

    let routes: Vec<Route> = vec![
        Route::builder()
            .method(Method::GET)
            .path("/employees")
            .handler_fn(root),
        Route::builder()
            .method(Method::GET)
            .path("/employees/:id")
            .handler_fn(employee_by_id),
        Route::builder()
            .method(Method::GET)
            .path("/employees/:id/assignments")
            .handler_fn(employee_by_id),
        Route::builder()
            .method(Method::GET)
            .path("/employees/:id/assignments/:assignment_id")
            .handler_fn(assignment_by_id),
        Route::builder()
            .method(Method::GET)
            .path("/albums/stunner")
            .handler_fn(crystal_logic),
        Route::builder()
            .method(Method::POST)
            .path("/albums")
            .handler_fn(thanks_for_album),
    ];

    Condey::init()
        .mount("/api/test", routes)
        .listen_at("127.0.0.1:3000")
        .await?;

    Ok(())
}
