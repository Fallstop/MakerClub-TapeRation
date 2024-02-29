// use crate::websocket::{connection_manager, Users};
use crate::api::openapi;
use axum::{
    http::{Method, Uri},
    routing::get,
    Router,
};
use sea_orm::DatabaseConnection;
use tokio::sync::broadcast::Sender;
use tower_http::cors::{AllowHeaders, AllowOrigin, CorsLayer};
use tracing::info;
use utoipa_redoc::{Redoc, Servable};
use websocket::WsMessage;

mod api;
mod db;
mod env_config;
mod names;
mod websocket;

struct AppState {
    tx: Sender<WsMessage>,
    db: DatabaseConnection,
}

#[tokio::main]
#[allow(clippy::needless_return)]
async fn main() -> Result<(), std::io::Error> {
    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", "info");
    }

    tracing_subscriber::fmt::init();

    info!("Initialising DB");
    let db = db::connect().await;

    info!("Starting server on http://localhost:8080/");

    let app = Router::new()
        .route("/openapi.yaml", get(openapi))
        .merge(Redoc::with_url("/docs", "/openapi.yaml"))
        .nest("/api", api::router())
        .fallback(|uri: Uri| async move {
            crate::api::error::Error::NotFound {
                resource: uri.path().to_string(),
            }
        })
        .with_state(db)
        .layer(
            CorsLayer::new()
                .allow_origin(AllowOrigin::any())
                .allow_methods([Method::GET, Method::POST, Method::PUT])
                .allow_headers(AllowHeaders::any()),
        );

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
    axum::serve(listener, app).await
}
