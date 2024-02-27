// use crate::websocket::{connection_manager, Users};
use axum::{extract::State, http::Uri, Router};
use tracing::info;

mod api;
mod db;
mod env_config;
mod names;
// mod websocket;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", "info");
    }

    tracing_subscriber::fmt::init();

    info!("Initialising DB");
    let db = db::connect().await;

    info!("Starting server on http://localhost:8080/");

    let app = Router::new()
        .nest("/api", api::router())
        .fallback(|uri: Uri| async move {
            crate::api::error::Error::NotFound {
                resource: uri.path().to_string(),
            }
        })
        .with_state(db);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
    axum::serve(listener, app).await
}
