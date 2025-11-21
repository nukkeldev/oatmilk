use askama::Template;
use axum::{
    Router,
    http::StatusCode,
    response::{Html, Response},
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

async fn table_handler() -> Html<String> {
    let mut table = "<table>".to_string();

    table.push_str("<tr><th>Title</th><th>Album</th><th>Author</th></tr>");
    for i in 0..100 {
        table += format!(
            "<tr><td>{title}</td><td>{album}</td><td>{author}</td></tr>",
            title = i,
            album = i,
            author = i
        )
        .as_str();
    }
    table.push_str("</table>");

    Html(table)
}

use axum::response::IntoResponse;

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
