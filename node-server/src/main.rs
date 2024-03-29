use std::collections::HashMap;

use tokio::sync::{mpsc, Mutex};

use warp::Filter;

use log::info;
use pretty_env_logger;
use rust_embed::RustEmbed;

mod keyboard_dev;
mod gpio_queue;
mod websocket;
mod env_config;
mod actions;
mod master_api;

use crate::actions::{State, StateInner};
use crate::gpio_queue::gpio_manager;
use crate::keyboard_dev::keyboard_manager;
use crate::websocket::connection_manager;

#[derive(RustEmbed)]
#[folder = "www_static/"]
struct StaticData;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    if let Err(_) = std::env::var("RUST_LOG") {
        std::env::set_var("RUST_LOG", "info");
    }
    pretty_env_logger::init();


    // GET /hello/warp => 200 OK with body "Hello, warp!"
    let static_dir = warp_embed::embed(&StaticData);
    let index_html = warp_embed::embed_one(&StaticData, "index.html");

    info!("Starting server on http://localhost:8081/");

    let (gpio_queue_sender, mut gpio_queue_receiver) = mpsc::channel(100);



    let state = State::new(Mutex::new(StateInner {
        websocket_stream: HashMap::new(),
        card_balance: None,
        card_nickname: None,
        card_id: None,
        gpio_channel: gpio_queue_sender,
    }));

    let gpio_state = state.clone();
    tokio::spawn(async move {
        gpio_manager(gpio_state, &mut gpio_queue_receiver).await.unwrap();
    });

    let keyboard_state = state.clone();
    tokio::spawn(async move {
        keyboard_manager(keyboard_state).await.unwrap();
    });


    let websocket = warp::path("stream")
        .and(warp::ws())
        .and(warp::any().map(move || state.clone()))
        .map(move |ws: warp::ws::Ws, users: State| {
            info!("New Websocket connection");

            // And then our closure will be called when it completes...
            ws.on_upgrade(move |socket| connection_manager(socket, users)) // Just echo all messages back...
        });


    warp::serve(static_dir.or(websocket).or(index_html))
        .run(([0, 0, 0, 0], 8081))
        .await;

    Ok(())
}
