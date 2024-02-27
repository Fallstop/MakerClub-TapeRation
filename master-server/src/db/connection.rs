use migration::MigratorTrait;
use sea_orm::{ConnectOptions, Database, DatabaseConnection};
use tracing::info;

use crate::env_config::ENV_CONFIG;

pub async fn connect() -> DatabaseConnection {
    let mut opt = ConnectOptions::new(&ENV_CONFIG.database_url);
    opt.sqlx_logging_level(log::LevelFilter::Info);

    let db = Database::connect(opt).await.unwrap();

    info!("Connected to DB, running migrations...");
    migration::Migrator::up(&db, None).await.unwrap();
    info!("Migrations complete");

    db
}
