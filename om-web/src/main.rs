use std::sync::Arc;

use askama::Template;
use axum::{
    Router,
    extract::State,
    http::StatusCode,
    response::{Html, IntoResponse, Response},
    routing::get,
};
use log::info;
use tokio::net::TcpListener;
use tower_http::services::ServeDir;

use om_core::db::Db;
use om_core::media::*;

#[derive(Debug, Clone)]
struct AppState {
    db: Arc<Db>,
}

#[derive(Template)]
#[template(path = "index.html")]
struct Index {}

#[tokio::main]
async fn main() {
    // Initialize the logging implementation.
    pretty_env_logger::init_timed();

    let db = Db::new_in_memory()
        .await
        .expect("Failed to connect to IN-MEMORY database...");

    let artist = db
        .add(Artist {
            name: "Aquilus".to_string(),
            description: Some("Solo Orchestral Atmospheric Black Metal Project".to_string()),
            location: Some("Australia".to_string()),
            ..Default::default()
        })
        .await
        .unwrap() as u32;

    let song = db
        .add(Song {
            title: "Nihil".to_string(),
            artist: artist,
            ..Default::default()
        })
        .await
        .unwrap();

    let state = AppState { db: Arc::new(db) };

    let app = Router::new()
        .route("/", get(index_handler))
        .route("/table", get(table_handler))
        .nest_service("/assets", ServeDir::new("build/assets/"))
        .with_state(state);

    let listener = TcpListener::bind("127.0.0.1:3000").await.unwrap();
    info!("Listening on {}...", listener.local_addr().unwrap());

    axum::serve(listener, app).await.unwrap();

    // TODO: Disconnect from the DB.
}

async fn index_handler() -> Result<impl IntoResponse, AppError> {
    Ok(Html(Index {}.render()?))
}

// -- Table -- //

#[derive(Template)]
#[template(path = "table.html")]
struct Table<'a> {
    songs: Vec<IndexSongView<'a>>,
}

#[derive(Debug)]
struct IndexSongView<'a> {
    title: &'a str,
    collection: Option<ID>,
    artist: ID,
}

async fn table_handler(State(state): State<AppState>) -> Result<impl IntoResponse, AppError> {
    let songs = state.db.get_all::<Song>().await.unwrap();
    let show_songs = songs
        .iter()
        .map(|s| IndexSongView {
            title: &s.title,
            collection: s.collection,
            artist: s.artist,
        })
        .collect::<Vec<IndexSongView<'_>>>();

    let table = Table { songs: show_songs };
    Ok(Html(table.render()?))
}

// -- Errors -- //

#[derive(Debug, displaydoc::Display, thiserror::Error)]
enum AppError {
    /// could not render template
    Render(#[from] askama::Error),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let status = match &self {
            AppError::Render(_) => StatusCode::INTERNAL_SERVER_ERROR,
        };
        (status, "Something went wrong").into_response()
    }
}
