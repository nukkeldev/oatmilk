use askama::Template;
use axum::{
    Router,
    http::StatusCode,
    response::{Html, IntoResponse, Response},
    routing::get,
};
use log::info;
use tokio::net::TcpListener;
use tower_http::services::ServeDir;

#[derive(Template)]
#[template(path = "index.html")]
struct Index {}

#[tokio::main]
async fn main() {
    // Initialize the logging implementation.
    pretty_env_logger::init_timed();

    let app = Router::new()
        .route("/", get(index_handler))
        .route("/table", get(table_handler))
        .nest_service("/assets", ServeDir::new("build/assets/"));

    let listener = TcpListener::bind("127.0.0.1:3000").await.unwrap();
    info!("Listening on {}...", listener.local_addr().unwrap());

    axum::serve(listener, app).await.unwrap();
}

async fn index_handler() -> Result<impl IntoResponse, AppError> {
    Ok(Html(Index {}.render()?))
}

// -- Table -- //

#[derive(Template)]
#[template(path = "table.html")]
struct Table<'a> {
    songs: Vec<Song<'a>>,
}

struct Song<'a> {
    title: &'a str,
    album: &'a str,
    author: &'a str,
}

async fn table_handler() -> Result<impl IntoResponse, AppError> {
    let mut table = Table { songs: vec![] };

    for _ in 0..100 {
        table.songs.push(Song {
            title: "Hello",
            album: "World",
            author: "John",
        });
    }

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
