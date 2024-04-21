use std::env;
use std::net::SocketAddr;
use std::sync::Arc;

use dotenvy::dotenv;
use sqlx::{MySql, Pool};
use tracing::info;

use crate::infra::database;

mod common;
mod controllers;
mod infra;
mod jobs;
mod middlewares;
mod models;
mod router;
mod services;

#[derive(Clone)]
pub struct AppState {
    pub db_pool: Arc<Pool<MySql>>,
}

/// Starts the Printerlynx Core Backend
pub async fn start() {
    match dotenv() {
        Ok(_) => {}
        Err(e) => {
            panic!("Error loading .env file: {}", e)
        }
    }
    
    tracing_subscriber::fmt().compact().with_target(true).init();

    info!("Starting Printerlynx Core...");

    output_system_info();

    let db_pool = database::get_pool().await;

    let state = Arc::new(AppState {
        db_pool: Arc::new(db_pool),
    });

    // init router and output addr information
    let app = router::api_v1::create(state).await;
    let port = 3000;
    let ip = [0, 0, 0, 0];
    let addr = SocketAddr::new(ip.into(), port);

    info!(
        "Server running at: http://localhost:{} || {}:{}",
        addr.port(),
        addr.ip(),
        addr.port()
    );
    axum::Server::bind(&addr)
        .serve(app.into_make_service_with_connect_info::<SocketAddr>())
        .await
        .expect("Failed to start Axum server")
}

pub fn output_system_info() {
    let operating_system = env::consts::OS;
    let architecture = env::consts::ARCH;
    let family = env::consts::FAMILY;

    info!(
        "System details: os: {}, arch: {}, fam: {}",
        operating_system, architecture, family
    );
}
