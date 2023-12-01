use std::env;
use std::net::SocketAddr;
use std::sync::Arc;

use dotenvy::dotenv;
use lapin::{Connection};
use sqlx::{MySql, Pool};
use tracing::info;

use crate::infra::{database, message_broker};

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
    pub amqp_connection: Arc<Connection>,
}

/// Starts the Printerlynx Core Backend
pub async fn start() {
    dotenv().expect(".env file not found");
    tracing_subscriber::fmt().compact().with_target(true).init();

    info!("Starting Printerlynx Core...");

    output_system_info();

    let db_pool = database::get_pool().await;
    let amqp_connection = message_broker::get_connection().await;

    let state = Arc::new(AppState {
        db_pool: Arc::new(db_pool),
        amqp_connection: Arc::new(amqp_connection),
    });


    // create the core que
    // let ch1 = state.amqp_connection.create_channel().await.unwrap();
    // ch1.queue_declare("test", QueueDeclareOptions::default(), Default::default()).await.expect("TODO: panic message");


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

/// Logs the operating system, architecture and family of the system
/// This is useful for debugging purposes
pub fn output_system_info() {
    let operating_system = env::consts::OS;
    let architecture = env::consts::ARCH;
    let family = env::consts::FAMILY;

    info!(
        "System details: os: {}, arch: {}, fam: {}",
        operating_system, architecture, family
    );
}

