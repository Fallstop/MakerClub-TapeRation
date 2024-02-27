use std::{
    collections::HashMap,
    sync::{
        atomic::{AtomicUsize, Ordering},
        Arc,
    },
};

use futures_util::{stream::SplitSink, SinkExt, StreamExt};
use log::error;
use serde::{Deserialize, Serialize};
use tokio::sync::Mutex;
use warp::filters::ws::{Message, WebSocket};

use crate::actions::State;

pub type WebSocketConnections = HashMap<usize, SplitSink<WebSocket, Message>>;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum UserPage {
    ScanCampusCard,
    CampusCardNotFound,
    TapeLengthSelection,
    RegistrationSuccessful,
    RegistrationExists,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UIStateUpdate {
    pub user_page: UserPage,
    pub card_nickname: Option<String>,
    pub card_id: Option<String>,
    pub card_balance: Option<f32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BarcodeScan {
    pub barcode_data: String,
}



fn get_id() -> usize {
    static COUNTER: AtomicUsize = AtomicUsize::new(1);
    COUNTER.fetch_add(1, Ordering::Relaxed)
}

pub async fn connection_manager(ws: WebSocket, state: State) {
    let user_id = get_id();
    let (user_ws_tx, mut user_ws_rx) = ws.split();

    state.lock().await.websocket_stream.insert(user_id, user_ws_tx);

    tokio::task::spawn(async move {
        while let Some(result) = user_ws_rx.next().await {
            match result {
                Ok(msg) => {
                    if msg.is_text() {
                        let msg = msg.to_str().unwrap();
                        let msg: BarcodeScan = serde_json::from_str(msg).unwrap();
                        state.lock().await.scan_card(&msg.barcode_data, false).await;
                    }
                }
                Err(e) => {
                    error!("Websocket error: {}", e);
                }
            }
        }

        // User has disconnected, cleanup
        state.lock().await.websocket_stream.remove(&user_id);
    });
}

pub async fn send_ui_update(websocket_stream: &mut WebSocketConnections, update: UIStateUpdate) {
    let update = Message::text(serde_json::to_string(&update).unwrap());
    for user in websocket_stream.values_mut() {
        match user.send(update.clone()).await {
            Ok(_) => {}
            Err(e) => {
                error!("Websocket error: {}", e);
            }
        }
    }
}