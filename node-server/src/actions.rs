use std::{collections::HashMap, sync::Arc};

use futures_util::stream::SplitSink;
use log::info;
use warp::filters::ws::{Message, WebSocket};
use tokio::sync::Mutex;

use crate::websocket::{send_ui_update, UIStateUpdate, WebSocketConnections};


pub type State = Arc<Mutex<StateInner>>;
pub struct StateInner {
    pub websocket_stream: WebSocketConnections,
    pub card_id: Option<String>,
    pub card_nickname: Option<String>,
    pub card_balance: Option<f32>,
}

impl StateInner {
    pub async fn scan_card(&mut self, card_id: &str, register_card: bool) {
        info!("Scanned card: {}", card_id);
        let card_data = match crate::master_api::check_card(card_id, register_card).await {
            Ok(card_data) => card_data,
            Err(e) => {
                info!("Error getting card data: {}", e);
                return;
            }
        };

        let ui_update = UIStateUpdate {
            user_page: crate::websocket::UserPage::TapeLengthSelection,
            card_id: Some(card_id.to_string()),
            card_nickname: Some(card_data.nick_name),
            card_balance: Some(card_data.tape_left_cm),
        };
        send_ui_update(&mut self.websocket_stream, ui_update).await;
    }
    
    pub async fn unscan_card(&mut self) {
        info!("Unscan card");
        self.card_id = None;
        self.card_nickname = None;
        self.card_balance = None;

        let ui_update = UIStateUpdate {
            user_page: crate::websocket::UserPage::ScanCampusCard,
            card_id: None,
            card_nickname: None,
            card_balance: None,
        };
        send_ui_update(&mut self.websocket_stream, ui_update).await;
    }
    
    pub async fn select_tape_length(&mut self, tape_length: f32) {
        info!("Selected length: {}", tape_length);

        let card_id = match &self.card_id {
            Some(card_id) => card_id,
            None => return,
        };

        let card_data = match crate::master_api::check_card(card_id, false).await {
            Ok(card_data) => card_data,
            Err(e) => {
                info!("Error getting card data: {}", e);
                return;
            }
        };

        if tape_length < card_data.tape_left_cm {
            // Valid Action!

            let ui_update = UIStateUpdate {
                user_page: crate::websocket::UserPage::TapeLengthSelection,
                card_id: Some(card_id.to_string()),
                card_nickname: Some(card_data.nick_name),
                card_balance: Some(card_data.tape_left_cm),
            };
            send_ui_update(&mut self.websocket_stream, ui_update).await;
        }

    }
}

