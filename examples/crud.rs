use condey::{
    http::Method,
    types::{Json, Path, Query},
    Condey, Route, State,
};

use anyhow::Result;
use serde::{Deserialize, Serialize};
use tokio::sync::Mutex;
use tracing::Level;

use std::{fmt::Display, sync::Arc};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, PartialOrd)]
pub struct Album {
    id: i32,
    band: String,
    name: String,
}

#[derive(Debug, Clone, Deserialize, PartialEq, PartialOrd)]
pub struct AlbumQuery {
    band: Option<String>,
    name: Option<String>,
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

#[tokio::main]
async fn main() -> Result<()> {
    let subscriber = tracing_subscriber::FmtSubscriber::builder()
        .with_max_level(Level::DEBUG)
        .finish();

    tracing::subscriber::set_global_default(subscriber)?;

    let all = Route::builder()
        .method(Method::GET)
        .path("/albums")
        .with_handler_fn(
            |query: Query<AlbumQuery>, db: State<Arc<Database>>| async move {
                let albums = db.inner().albums.lock().await;

                let queried_albums = albums
                    .iter()
                    .filter(|album| {
                        query
                            .name
                            .as_ref()
                            .map(|name| album.name == *name)
                            .unwrap_or(true)
                    })
                    .filter(|album| {
                        query
                            .band
                            .as_ref()
                            .map(|band| album.band == *band)
                            .unwrap_or(true)
                    })
                    .cloned()
                    .collect::<Vec<_>>();

                Json(queried_albums)
            },
        );

    let get_by_id = Route::builder()
        .method(Method::GET)
        .path("/albums/:id")
        .with_handler_fn(
            |Path((id,)): Path<(i32,)>, db: State<Arc<Database>>| async move {
                let albums = db.inner().albums.lock().await;

                albums
                    .iter()
                    .find(|album| album.id == id)
                    .cloned()
                    .map(Json)
            },
        );

    let create = Route::builder()
        .method(Method::POST)
        .path("/albums")
        .with_handler_fn(
            |new: Json<AlbumWritable>, db: State<Arc<Database>>| async move {
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
            },
        );

    let update =
        Route::builder()
            .method(Method::PUT)
            .path("/albums/:id")
            .with_handler_fn(
                |Path((id,)): Path<(i32,)>,
                 db: State<Arc<Database>>,
                 updated: Json<AlbumWritable>| async move {
                    let updated = updated.into_inner();
                    let mut albums = db.inner().albums.lock().await;

                    let album = albums.iter_mut().find(|album| album.id == id)?;
                    album.name = updated.name;
                    album.band = updated.band;

                    Some(Json(album.clone()))
                },
            );

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
