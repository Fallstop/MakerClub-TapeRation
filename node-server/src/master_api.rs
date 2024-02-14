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

#[derive(Debug, Clone, Serialize, Deserialize)]
struct ApiError {
    pub error_message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
enum ApiResult {
    Ok(CardData),
    Error(ApiError),
}

pub async fn check_card(card_id: &str) -> Result<CardData, String> {
    let base_url = ENV_CONFIG.master_url.to_owned();

    let client = reqwest::Client::new();
    
    let res = client.get(format!("{base_url}/api/campus_card/{card_id}"))
        .send()
        .await
        .unwrap();
    let data: ApiResult = res.json().await.map_err(|e| e.to_string())?;
    println!("{:?}", data);

    match data {
        ApiResult::Ok(card_data) => Ok(card_data),
        ApiResult::Error(error) => Err(error.error_message),
    }
}