use std::{collections::HashMap, sync::Arc};

use futures_util::stream::SplitSink;
use log::info;
use warp::filters::ws::{Message, WebSocket};
use tokio::sync::Mutex;


pub type State = Arc<Mutex<StateInner>>;
pub struct StateInner {
    pub websocket_stream: HashMap<usize, SplitSink<WebSocket, Message>>,
    pub card_id: Option<String>,
    pub card_balance: Option<f32>,
}

impl StateInner {
    pub async fn scan_card(&self, card_id: &str) {
        info!("Scanned card: {}", card_id);
    
    }
    
    pub async fn unscan_card(&self) {
        info!("Unscan card");
    }
    
    pub async fn select_tape_length(&self, tape_length: usize) {
        info!("Selected length: {}", tape_length);
    }
}

