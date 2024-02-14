use crate::websocket::{connection_manager, Users};
use log::info;
use std::collections::HashMap;
use tokio::sync::Mutex;
use warp::{reject::Rejection, Filter};

mod api;
mod db;
mod env_config;
mod websocket;

#[cfg(feature = "serve_static")]
#[derive(rust_embed::RustEmbed)]
#[folder = "www_static/"]
struct StaticData;

#[cfg(feature = "serve_static")]
async fn serve<T, U>(api: T, websocket: U)
where
    T: Filter<Error = Rejection> + Sync + Send + Clone + 'static,
    T::Extract: warp::reply::Reply,
    U: Filter<Error = Rejection> + Sync + Send + Clone + 'static,
    U::Extract: warp::reply::Reply,
{
    // GET /hello/warp => 200 OK with body "Hello, warp!"
    let static_dir = warp_embed::embed(&StaticData);
    let index_html = warp_embed::embed_one(&StaticData, "index.html");

    warp::serve(
        api.or(static_dir)
            .or(websocket)
            .or(index_html)
            .or(static_dir),
    )
    .run(([0, 0, 0, 0], 8080))
    .await;
}

#[cfg(not(feature = "serve_static"))]
async fn serve<T, U>(api: T, websocket: U)
where
    T: Filter<Error = Rejection> + Sync + Send + Clone + 'static,
    T::Extract: warp::reply::Reply,
    U: Filter<Error = Rejection> + Sync + Send + Clone + 'static,
    U::Extract: warp::reply::Reply,
{
    warp::serve(api.or(websocket))
        .run(([0, 0, 0, 0], 8080))
        .await;
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", "info");
    }

    pretty_env_logger::init();

    info!("Initialising DB");
    let db = db::connect().await;

    info!("Starting server on http://localhost:8080/");

    let users = Users::new(Mutex::new(HashMap::new()));

    let websocket = warp::path("stream")
        .and(warp::ws())
        .and(warp::any().map(move || users.clone()))
        .map(move |ws: warp::ws::Ws, users: Users| {
            info!("New Websocket connection");

            // And then our closure will be called when it completes...
            ws.on_upgrade(move |socket| connection_manager(socket, users))
        });

    let api = api::create_warp_route(db);

    serve(api, websocket).await;

    Ok(())
}
