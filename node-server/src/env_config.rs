use std::env;

use log::info;

use once_cell::sync::Lazy;

#[derive(PartialEq, Debug, Clone)]
pub enum NodeType {
    TapeDispenser,
    CardRegister
}

pub struct EnvConfig {
    pub master_url: String,
    pub node_type: NodeType,
    pub tape_lengths_cm: Vec<f32>,
    pub master_api_password: String
}

pub static ENV_CONFIG: Lazy<EnvConfig> = Lazy::new(|| {
    if dotenv::dotenv().is_err() {
        info!("No .env file found");
    }

    let tape_lengths_raw = env::var("TAPE_LENGTHS").expect("Missing TAPE_LENGTHS environment variable");
    let tape_lengths: Vec<f32> = tape_lengths_raw.split(",").map(|s| s.trim().parse().unwrap()).collect();

    EnvConfig {
        master_url: env::var("MASER_SERVER_URL")
            .expect("Missing MASER_SERVER_URL environment variable"),
        node_type: match env::var("NODE_TYPE").expect("Missing NODE_TYPE environment variable").as_str() {
            "tape_dispenser" => NodeType::TapeDispenser,
            "card_register" => NodeType::CardRegister,
            _ => panic!("Invalid NODE_TYPE environment variable")
        },
        tape_lengths_cm: tape_lengths,
        master_api_password: env::var("PASSWORD")
            .expect("Missing PASSWORD environment variable")
    }
});
