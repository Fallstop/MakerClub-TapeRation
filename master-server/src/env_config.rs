use std::env;

use log::info;

use once_cell::sync::Lazy;

pub struct EnvConfig {
    pub db_connection_uri: String,
}

pub static ENV_CONFIG: Lazy<EnvConfig> = Lazy::new(|| {
    if dotenv::dotenv().is_err() {
        info!("No .env file found");
    }

    EnvConfig {
        db_connection_uri: env::var("DATABASE_CONNECTION_URI")
            .expect("Missing DATABASE_CONNECTION_URI environment variable"),
    }
});
