use std::env;

use log::info;

use once_cell::sync::Lazy;

pub enum NodeType {
    TapeDispenser,
    CardRegister
}

pub struct EnvConfig {
    pub master_url: String,
    pub node_type: NodeType
}

pub static ENV_CONFIG: Lazy<EnvConfig> = Lazy::new(|| {
    if dotenv::dotenv().is_err() {
        info!("No .env file found");
    }

    EnvConfig {
        master_url: env::var("MASER_SERVER_URL")
            .expect("Missing MASER_SERVER_URL environment variable"),
        node_type: match env::var("NODE_TYPE").expect("Missing NODE_TYPE environment variable").as_str() {
            "tape_dispenser" => NodeType::TapeDispenser,
            "card_register" => NodeType::CardRegister,
            _ => panic!("Invalid NODE_TYPE environment variable")
        }
    }
});
