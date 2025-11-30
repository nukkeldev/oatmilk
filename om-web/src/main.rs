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
use tokio::{net::TcpListener, signal};
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

    let db = Db::new_at_url("sqlite:oatmilk.db")
        .await
        .expect("Failed to connect to database...");

    let artist = db
        .add(Artist {
            name: "Aquilus".to_string(),
            description: Some("Solo Orchestral Atmospheric Black Metal Project".to_string()),
            location: Some("Australia".to_string()),
            ..Default::default()
        })
        .await
        .unwrap() as u32;

    let track = db
        .add(Track {
            title: "Nihil".to_string(),
            artist: artist,
            ..Default::default()
        })
        .await
        .unwrap();

    let state = Arc::new(AppState { db: Arc::new(db) });

    let app = Router::new()
        .route("/", get(index_handler))
        .route("/search", get(search_handler))
        .nest_service("/assets", ServeDir::new("build/"))
        .with_state(state.clone());

    let listener = TcpListener::bind("127.0.0.1:3000").await.unwrap();
    info!("Listening on {}...", listener.local_addr().unwrap());

    let server = axum::serve(listener, app);

    tokio::select! {
        _ = server => {},
        _ = shutdown_signal() => {
            state.db.disconnect().await;
            println!("Shutting down...");
        }
    }
}

async fn index_handler() -> Result<impl IntoResponse, AppError> {
    Ok(Html(Index {}.render()?))
}

// -- Table -- //

#[derive(Template)]
#[template(path = "results.html")]
struct Results {
    tracks: Vec<Track>,
}

async fn search_handler(State(state): State<Arc<AppState>>) -> Result<impl IntoResponse, AppError> {
    let tracks = state.db.get_all::<Track>().await.unwrap();
    let table = Results { tracks };
    Ok(Html(table.render()?))
}

impl Results {
    fn format_duration(&self, mut seconds: &Option<u32>) -> String {
        if let Some(dur) = seconds.and_then(|s| chrono::Duration::new(s as i64, 0)) {
            format!(
                "{:02}:{:02}:{:02}",
                dur.num_hours(),
                dur.num_minutes() % 60,
                dur.num_seconds() % 60
            )
        } else {
            "--:--:--".to_string()
        }
    }
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

async fn shutdown_signal() {
    signal::ctrl_c()
        .await
        .expect("failed to install CTRL+C handler");
}
