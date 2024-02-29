use std::{error::Error, fmt::Display};

use serde::{Deserialize, Serialize};

use crate::env_config::ENV_CONFIG;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CardData {
    pub id: i32,
    pub campus_card: String,
    pub nick_name: String,
    pub date_registered: String,
    pub last_transaction: Option<String>,
    pub tape_left_cm: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TapeData {
    pub tape_left_cm: f32,
}



#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DispenseTape {
    pub tape_length_cm: f32,
    pub unix_timestamp: usize,
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
    OkCard(CardData),
    OkTape(TapeData),
    Error(ApiError),
}

pub async fn check_card(card_id: &str, register_card: bool) -> Result<CardData, Box<dyn Error + Send + Sync>> {
    let base_url = ENV_CONFIG.master_url.to_owned();

    let client = reqwest::Client::new();

    let mut base_request = match register_card {
        true => client.put(format!("{base_url}/api/campus_card/{card_id}")),
        false => client.get(format!("{base_url}/api/campus_card/{card_id}")),
    };
    base_request = base_request.header("password", ENV_CONFIG.master_api_password.to_owned());
    
    let res = base_request
        .send()
        .await?;
    let data: ApiResult = res.json().await.map_err(|e| e.to_string())?;
    println!("{:?}", data);

    match data {
        ApiResult::OkCard(card_data) => Ok(card_data),
        ApiResult::Error(error) => Err(Box::new(error)),
        _=>{unreachable!()}
    }
}

pub async fn dispense_tape(card_id: &str, tape_length_cm: f32) -> Result<TapeData, Box<dyn Error + Send + Sync>> {
    let base_url = ENV_CONFIG.master_url.to_owned();

    let client = reqwest::Client::new();

    let res = client
        .post(format!("{base_url}/api/campus_card/{card_id}/add"))
        .header("password", ENV_CONFIG.master_api_password.to_owned())
        .query(&[("tape_cm", -tape_length_cm)])
        .send()
        .await?;
    let text: String = res.text().await?;
    println!("{:?}", text);
    let data: ApiResult = serde_json::from_str(&text).map_err(|e| e.to_string())?;

    // let data: ApiResult = res.json().await.map_err(|e| e.to_string())?;

    match data {
        ApiResult::OkTape(tape_data) =>Ok(tape_data),
        ApiResult::Error(error) => Err(Box::new(error)),
        _=>{unreachable!()}
    }
}