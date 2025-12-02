use std::sync::Arc;

use askama::Template;
use axum::{
    Form, Router,
    extract::{Query, State},
    http::{HeaderMap, StatusCode},
    response::{Html, IntoResponse, Redirect, Response},
    routing::{get, post},
};
use log::{debug, info};
use serde::Deserialize;
use tokio::{net::TcpListener, signal};
use tower_http::services::ServeDir;

use om_core::{
    db::{Db, SQLxError, SearchResult},
    media::*,
};

#[derive(Debug, Clone)]
struct AppState {
    db: Arc<Db>,
}

#[tokio::main]
async fn main() {
    // Initialize the logging implementation.
    pretty_env_logger::init_timed();

    let db = Db::new_at_url("sqlite:oatmilk.db")
        .await
        .expect("Failed to connect to database...");

    let state = Arc::new(AppState { db: Arc::new(db) });

    let app = Router::new()
        .route("/", get(index_handler))
        .route("/search", get(search_handler))
        .route("/new", get(new_handler))
        .route("/new/track", post(new_track_handler))
        .route("/new/artist", post(new_artist_handler))
        .route("/new/collection", post(new_collection_handler))
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

// -- Index -- //

async fn index_handler() -> impl IntoResponse {
    Redirect::permanent("/search")
}

// -- Base -- //

#[derive(Template)]
#[template(path = "base.html")]
struct Based {
    page_name: String,
    content: String,
}

impl Based {
    pub fn format_page_css(&self) -> String {
        format!("href=\"/assets/styles/{}.css\"", self.page_name)
    }
}

// -- Search -- //

#[derive(Template)]
#[template(path = "search.html")]
struct Search {
    query: Option<String>,
    result: SearchResult<Track>,
}

#[derive(Debug, Deserialize)]
struct SearchQuery {
    query: Option<String>,
}

async fn search_handler(
    headers: HeaderMap,
    State(state): State<Arc<AppState>>,
    Query(uri_query): Query<SearchQuery>,
) -> Result<impl IntoResponse, AppError> {
    let fragment = is_htmx(headers);

    let (result, query) = if let Some(query) = uri_query.query
        && !query.is_empty()
    {
        (
            state
                .db
                .get::<Track>(format!("WHERE {}", query).as_str())
                .await,
            Some(query),
        )
    } else {
        (state.db.get_all::<Track>().await, None)
    };

    debug!("Rendering search/ with query: {:?}", query);

    let mut out = Search { result, query }.render()?;
    if !fragment {
        out = Based {
            content: out,
            page_name: "search".to_string(),
        }
        .render()?;
    }
    Ok(Html(out))
}

impl Search {
    fn format_duration(&self, seconds: &Option<u32>) -> String {
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

    fn format_query(&self) -> String {
        self.query.clone().unwrap_or(String::new())
    }
}

// -- New -- //

#[derive(Template)]
#[template(path = "new.html")]
struct New {
    variant: String,
}

#[derive(Debug, Deserialize)]
struct NewInput {
    variant: Option<String>,
}

async fn new_handler(
    headers: HeaderMap,
    Form(input): Form<NewInput>,
) -> Result<impl IntoResponse, AppError> {
    let fragment = is_htmx(headers);

    let variant = if let Some(variant) = input.variant
        && !variant.is_empty()
    {
        variant
    } else {
        "track".to_string()
    };

    let mut out = New { variant }.render()?;
    if !fragment {
        out = Based {
            content: out,
            page_name: "new".to_string(),
        }
        .render()?;
    }
    Ok(Html(out))
}

impl New {
    fn is_variant_selected(&self, target: &str) -> &str {
        if self.variant.as_str() == target {
            "selected"
        } else {
            ""
        }
    }
}

// -- New -- //

async fn new_track_handler(
    State(state): State<Arc<AppState>>,
    Form(input): Form<NewTrack>,
) -> Result<impl IntoResponse, AppError> {
    let id = state.db.add::<Track>(input).await?;
    Ok(id.to_string())
}

async fn new_artist_handler(
    State(state): State<Arc<AppState>>,
    Form(input): Form<NewArtist>,
) -> Result<impl IntoResponse, AppError> {
    let id = state.db.add::<Artist>(input).await?;
    Ok(id.to_string())
}

async fn new_collection_handler(
    State(state): State<Arc<AppState>>,
    Form(input): Form<NewCollection>,
) -> Result<impl IntoResponse, AppError> {
    let id = state.db.add::<Collection>(input).await?;
    Ok(id.to_string())
}

// -- Utils -- //

fn is_htmx(headers: HeaderMap) -> bool {
    headers
        .get("HX-Request")
        .and_then(|v| v.to_str().ok())
        .map(|s| s == "true")
        .unwrap_or(false)
}

// -- Errors -- //

#[derive(Debug, displaydoc::Display, thiserror::Error)]
enum AppError {
    /// Could not render template: {0}
    Render(#[from] askama::Error),
    /// SQL Error: {0}
    SQLx(#[from] SQLxError),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let status = StatusCode::INTERNAL_SERVER_ERROR;
        /*match &self {
            AppError::Render(_) => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::NoProvidedContentRoute => todo!(),
            AppError::UnknownRoute(_) => todo!(),
        };*/
        (status, "Something went wrong").into_response()
    }
}

async fn shutdown_signal() {
    signal::ctrl_c()
        .await
        .expect("failed to install CTRL+C handler");
}
