use std::{fmt::Display, sync::Arc};

use anyhow::Result;
use condey::{
    http::Method,
    types::{Json, Path},
    Condey, Route, State,
};
use serde::{Deserialize, Serialize};
use tokio::sync::Mutex;
use tracing::Level;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, PartialOrd)]
pub struct Album {
    id: i32,
    band: String,
    name: String,
}

impl Album {
    pub fn new<S1: Display, S2: Display>(id: i32, band: S1, name: S2) -> Self {
        Album {
            id,
            band: band.to_string(),
            name: name.to_string(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, PartialOrd)]
pub struct AlbumWritable {
    band: String,
    name: String,
}

#[derive(Debug)]
pub struct Database {
    albums: Mutex<Vec<Album>>,
}

impl Database {
    pub fn with_sample_data(albums: Vec<Album>) -> Self {
        Self {
            albums: Mutex::new(albums),
        }
    }
}

async fn all(db: State<Arc<Database>>) -> Json<Vec<Album>> {
    let albums = db.inner().albums.lock().await;

    Json(albums.clone())
}

async fn get_by_id(db: State<Arc<Database>>, Path((id,)): Path<(i32,)>) -> Option<Json<Album>> {
    let albums = db.inner().albums.lock().await;

    albums
        .iter()
        .find(|album| album.id == id)
        .cloned()
        .map(Json)
}

async fn update(
    db: State<Arc<Database>>,
    Path((id,)): Path<(i32,)>,
    updated: Json<AlbumWritable>,
) -> Option<Json<Album>> {
    let updated = updated.into_inner();
    let mut albums = db.inner().albums.lock().await;

    let album = albums.iter_mut().find(|album| album.id == id)?;
    album.name = updated.name;
    album.band = updated.band;

    Some(Json(album.clone()))
}

async fn create(db: State<Arc<Database>>, new: Json<AlbumWritable>) -> Json<Album> {
    let new = new.into_inner();
    let mut albums = db.inner().albums.lock().await;

    let new_id = albums.iter().map(|album| album.id).max().unwrap_or(1);

    let new_album = Album {
        id: new_id,
        band: new.band,
        name: new.name,
    };

    albums.push(new_album.clone());

    Json(new_album)
}

#[tokio::main]
async fn main() -> Result<()> {
    let subscriber = tracing_subscriber::FmtSubscriber::builder()
        .with_max_level(Level::DEBUG)
        .finish();

    tracing::subscriber::set_global_default(subscriber)?;

    let all = Route::builder()
        .method(Method::GET)
        .path("/albums")
        .with_handler_fn(all);

    let get_by_id = Route::builder()
        .method(Method::GET)
        .path("/albums/:id")
        .with_handler_fn(get_by_id);

    let create = Route::builder()
        .method(Method::POST)
        .path("/albums")
        .with_handler_fn(create);

    let update = Route::builder()
        .method(Method::PUT)
        .path("/albums/:id")
        .with_handler_fn(update);

    let albums = vec![
        Album::new(1, "Manilla Road", "Crystal Logic"),
        Album::new(2, "Cirith Ungol", "Frost And Fire"),
        Album::new(3, "Turbo", "Dorosłe Dzieci"),
        Album::new(4, "Exodus", "Bonded By Blood"),
        Album::new(5, "Crimson Glory", "Crimson Glory"),
    ];

    let database = Database::with_sample_data(albums);

    Condey::init()
        .mount("/api", vec![all, get_by_id, create, update])
        .app_state(Arc::new(database))
        .listen_at("127.0.0.1:3000")
        .await?;

    Ok(())
}
