use std::{error::Error, fmt::Display};

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
impl Display for ApiError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ApiError: {}", self.error_message)
    }
}
impl Error for ApiError {

}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
enum ApiResult {
    Ok(CardData),
    Error(ApiError),
}

pub async fn check_card(card_id: &str, register_card: bool) -> Result<CardData, Box<dyn Error + Send + Sync>> {
    let base_url = ENV_CONFIG.master_url.to_owned();

    let client = reqwest::Client::new();

    let base_request = match register_card {
        true => client.put(format!("{base_url}/api/campus_card/{card_id}")),
        false => client.get(format!("{base_url}/api/campus_card/{card_id}")),
    };
    
    let res = base_request
        .send()
        .await?;
    let data: ApiResult = res.json().await.map_err(|e| e.to_string())?;
    println!("{:?}", data);

    match data {
        ApiResult::Ok(card_data) => Ok(card_data),
        ApiResult::Error(error) => Err(Box::new(error)),
    }
}