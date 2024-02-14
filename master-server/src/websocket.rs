use std::{
    collections::HashMap,
    sync::{
        atomic::{AtomicUsize, Ordering},
        Arc,
    },
};

use futures_util::{stream::SplitSink, SinkExt, StreamExt};
use log::error;
use tokio::sync::Mutex;
use warp::filters::ws::{Message, WebSocket};

pub type Users = Arc<Mutex<HashMap<usize, SplitSink<WebSocket, Message>>>>;

fn get_id() -> usize {
    static COUNTER: AtomicUsize = AtomicUsize::new(1);
    COUNTER.fetch_add(1, Ordering::Relaxed)
}

pub async fn connection_manager(ws: WebSocket, users: Users) {
    let user_id = get_id();
    let (user_ws_tx, mut user_ws_rx) = ws.split();

    users.lock().await.insert(user_id, user_ws_tx);

    tokio::task::spawn(async move {
        while let Some(result) = user_ws_rx.next().await {
            match result {
                Ok(msg) => {
                    for user in users.lock().await.values_mut() {
                        match user.send(msg.clone()).await {
                            Ok(_) => {}
                            Err(e) => {
                                error!("Websocket error: {}", e);
                            }
                        }
                    }
                }
                Err(e) => {
                    error!("Websocket error: {}", e);
                }
            }
        }

        // User has disconnected, cleanup
        users.lock().await.remove(&user_id);
    });
}
