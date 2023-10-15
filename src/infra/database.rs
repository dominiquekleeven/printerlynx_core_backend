use std::env;

use sqlx::migrate::MigrateDatabase;
use sqlx::mysql::MySqlPoolOptions;
use sqlx::{MySql, MySqlPool};
use tracing::{error, info};

/// Retrieves the database pool connection
pub async fn get_pool() -> MySqlPool {
    let path = env::var("DATABASE_URL").expect("DATABASE_URL not found in .env file");
    info!("Connecting to database...");

    if !MySql::database_exists(&path).await.unwrap_or(false) {
        error!("Database does not exist or could not be found, please run the migrations");
        panic!("Database does not exist or could not be found, please run the migrations");
    }

    let options = MySqlPoolOptions::new()
        .max_connections(100)
        .min_connections(5)
        .acquire_timeout(std::time::Duration::from_secs(30))
        .idle_timeout(std::time::Duration::from_secs(30));

    match options.connect(&path).await {
        Ok(pool) => {
            info!("Connected to database");
            pool
        }
        Err(e) => {
            info!("Error connecting to database: {}", e);
            panic!("Error connecting to database");
        }
    }
}
