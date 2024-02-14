use serde::{
    Deserialize, Serialize,
};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ErrorResponse {
    pub error_message: String
}