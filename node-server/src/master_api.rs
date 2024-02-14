use serde::{Deserialize, Serialize};

use crate::env_config::ENV_CONFIG;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CardData {
    pub id: i32,
    pub nick_name: String,
    pub date_registered: String,
    pub last_transaction: Option<String>,
    pub tape_left_cm: f32,
}


pub async fn check_card(card_id: &str) -> CardData {
    let base_url = ENV_CONFIG.master_url.to_owned();

    let client = reqwest::Client::new();
    
    let res = client.get(format!("{base_url}/api/campus_card/{card_id}"))
        .send()
        .await
        .unwrap();
    // let data: CardData = res.json().await.unwrap();
    // println!("{:?}", data);

    return CardData {
        id: 0,
        nick_name: "Wow What".to_string(),
        date_registered: "2024-10-20T03:20:24.2121".to_string(),
        last_transaction: None,
        tape_left_cm: 100.0,
    }
}