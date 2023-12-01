use std::env;
use std::sync::Arc;

use lapin::{Channel, Connection, ConnectionProperties};
use tracing::{error, info};
use crate::common::app_error::AppError;

pub async fn get_connection() -> Connection {
    let addr = env::var("AMQP_URL").expect("AMQP_URL must be set!");
    info!("Connecting to message broker...");

    match Connection::connect(&addr, ConnectionProperties::default()).await {
        Ok(conn) => {
            info!("Connected to message broker");
            conn
        }
        Err(e) => {
            panic!("Error connecting to message broker: {}", e);
        }
    }
}

