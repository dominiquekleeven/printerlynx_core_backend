use lapin::{Connection, ConnectionProperties};
use std::env;
use tracing::info;

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
