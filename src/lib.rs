use std::env;
use std::net::SocketAddr;
use std::sync::Arc;

use dotenvy::dotenv;
use sqlx::{MySql, Pool};
use tokio::sync::Mutex;
use tracing::info;

use crate::infra::database;
use crate::infra::websockets::WebSocketSession;

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
    pub pool: Arc<Pool<MySql>>,
}

#[allow(dead_code)]
#[derive(Clone)]
pub struct AppWebSocketState {
    pub sessions: Arc<Mutex<Vec<WebSocketSession>>>,
}

/// Starts the Printerlynx Backend server
/// - Loads the .env file
/// - Initializes the logger
/// - Logs the operating system, architecture and family of the system
/// - Initializes the router
/// - Starts the server
pub async fn start() {
    dotenv().expect(".env file not found");
    tracing_subscriber::fmt().compact().with_target(true).init();

    info!("Starting up...");
    info_system();

    let pool = database::get_pool().await;
    let state = Arc::new(AppState {
        pool: Arc::new(pool),
    });

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

/// Logs the operating system, architecture and family of the system
/// This is useful for debugging purposes
pub fn info_system() {
    let operating_system = env::consts::OS;
    let architecture = env::consts::ARCH;
    let family = env::consts::FAMILY;

    info!(
        "System details: os: {}, arch: {}, fam: {}",
        operating_system, architecture, family
    );
}

#[allow(dead_code)]
pub fn check_env() -> bool {
    false
}
