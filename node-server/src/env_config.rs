use std::env;

use log::info;

use once_cell::sync::Lazy;

pub struct EnvConfig {
    pub master_url: String,
}

pub static ENV_CONFIG: Lazy<EnvConfig> = Lazy::new(|| {
    if dotenv::dotenv().is_err() {
        info!("No .env file found");
    }

    EnvConfig {
        master_url: env::var("MASER_SERVER_URL")
            .expect("Missing MASER_SERVER_URL environment variable"),
    }
});
