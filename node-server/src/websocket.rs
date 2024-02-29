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

use crate::{actions::State, env_config::ENV_CONFIG};

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
    pub tape_lengths_cm: Vec<f32>,
    pub card_nickname: Option<String>,
    pub card_id: Option<String>,
    pub card_balance: Option<f32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BarcodeScan {
    pub barcode_data: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogoutCard {

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DispenseTape {
    pub tape_length_cm: f32,
    pub unix_timestamp: usize,
}

#[derive(Serialize, Deserialize)]
enum IncomingWebsocketMessage {
    BarcodeScan(BarcodeScan),
    LogoutCard(LogoutCard),
    DispenseTape(DispenseTape)
}


fn get_id() -> usize {
    static COUNTER: AtomicUsize = AtomicUsize::new(1);
    COUNTER.fetch_add(1, Ordering::Relaxed)
}

pub async fn connection_manager(ws: WebSocket, state: State) {
    let user_id = get_id();
    let (mut user_ws_tx, mut user_ws_rx) = ws.split();

    user_ws_tx.send(Message::text(
        serde_json::to_string(&UIStateUpdate {
            user_page: UserPage::ScanCampusCard,
            tape_lengths_cm: ENV_CONFIG.tape_lengths_cm.clone(),
            card_nickname: None,
            card_id: None,
            card_balance: None,
        })
        .unwrap(),
    )).await.unwrap();

    state.lock().await.websocket_stream.insert(user_id, user_ws_tx);

    tokio::task::spawn(async move {
        while let Some(result) = user_ws_rx.next().await {
            match result {
                Ok(msg) => {
                    if !msg.is_text() {
                        return;
                    }

                    let msg: &str = msg.to_str().unwrap();
                    let msg: IncomingWebsocketMessage = serde_json::from_str(msg).unwrap();
                    action_websocket_message(msg, state.clone()).await;
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

async fn action_websocket_message(message: IncomingWebsocketMessage, state: State) {
    match message {
        IncomingWebsocketMessage::BarcodeScan(scan) => {
            state.lock().await.scan_card(&scan.barcode_data).await;
        }
        IncomingWebsocketMessage::LogoutCard(_) => {
            state.lock().await.unscan_card().await;
        }
        IncomingWebsocketMessage::DispenseTape(tape_details) => {
            println!("Dispensing tape: {:?}", tape_details);
            state.lock().await.select_tape_length(tape_details.tape_length_cm).await;
        }
    }
}