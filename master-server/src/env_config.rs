use std::env;

use tracing::info;

use once_cell::sync::Lazy;

pub struct EnvConfig {
    pub database_url: String,
    pub password: String,
}

pub static ENV_CONFIG: Lazy<EnvConfig> = Lazy::new(|| {
    if dotenv::dotenv().is_err() {
        info!("No .env file found");
    }

    EnvConfig {
        database_url: env::var("DATABASE_URL")
            .expect("Missing DATABASE_CONNECTION_URI environment variable"),
        password: env::var("PASSWORD").expect("Missing PASSWORD environment variable"),
    }
});
