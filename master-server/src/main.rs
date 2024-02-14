use std::collections::HashMap;

use tokio::sync::Mutex;

use warp::Filter;

use log::info;
use pretty_env_logger;
use rust_embed::RustEmbed;

mod websocket;
mod db;
mod api;
mod env_config;

use crate::websocket::{connection_manager, Users};

#[derive(RustEmbed)]
#[folder = "www_static/"]
struct StaticData;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    if let Err(_) = std::env::var("RUST_LOG") {
        std::env::set_var("RUST_LOG", "info");
    }
    pretty_env_logger::init();

    info!("Initialising DB");
    let db = db::connect().await;


    // GET /hello/warp => 200 OK with body "Hello, warp!"
    let static_dir = warp_embed::embed(&StaticData);
    let index_html = warp_embed::embed_one(&StaticData, "index.html");

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

    warp::serve(api.or(static_dir).or(websocket).or(index_html))
        .run(([0, 0, 0, 0], 8080))
        .await;

    Ok(())
}
