use axum::{
    extract::{
        ws::{WebSocket, WebSocketUpgrade},
        State,
    },
    response::Response,
};
use tracing::error;

use crate::AppState;

#[derive(serde::Serialize, Clone, Debug, PartialEq)]
pub struct SimplifiedParticipant {
    nick_name: String,
    amount: f64,
}

#[derive(serde::Serialize, Clone, Debug, PartialEq)]
pub enum WsMessage {
    TransactionAdd {
        nick_name: String,
        amount: f64,
    },
    TransactionSet {
        nick_name: String,
        amount: f64,
    },
    Join {
        nick_name: String,
        amount: f64,
    },
    Setup {
        participants: Vec<SimplifiedParticipant>,
    },
}

fn handler(ws: WebSocketUpgrade, state: State<AppState>) -> Response {
    ws.on_upgrade(|ws| handle_socket(ws, state))
}

async fn handle_socket(mut socket: WebSocket, State(AppState { tx, .. }): State<AppState>) {
    let mut reader = tx.subscribe();
    while let Ok(msg) = reader.recv().await {
        match serde_json::to_string(&msg) {
            Ok(item) => {
                if let Err(ex) = socket.send(axum::extract::ws::Message::Text(item)).await {}
            }
            Err(_) => todo!(),
        }
    }

    if let Err(ex) = socket.close().await {
        error!("Could not close web socket connection {ex}");
    }
}
