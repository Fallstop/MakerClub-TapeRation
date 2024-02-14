use std::env;

use log::info;
use migration::MigratorTrait;
use sea_orm::{ConnectOptions, Database, DatabaseConnection};
use warp::Filter;

use crate::env_config::ENV_CONFIG;

pub async fn connect() -> DatabaseConnection {
    let mut opt = ConnectOptions::new(&ENV_CONFIG.db_connection_uri);
    opt.sqlx_logging_level(log::LevelFilter::Debug);
    
    let db = Database::connect(opt).await.unwrap();

    info!("Connected to DB, running migrations...");
    migration::Migrator::up(&db, None).await.unwrap();
    info!("Migrations complete");

    db
}


pub fn with_db(db: DatabaseConnection) -> impl Filter<Extract = (DatabaseConnection,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || db.clone())
}
