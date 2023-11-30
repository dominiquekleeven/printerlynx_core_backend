use std::env;

use lapin::{Channel, Connection, ConnectionProperties};
use tracing::info;

pub async fn get_channel() -> Channel {
    let addr = env::var("AMQP_URL").expect("AMQP_URL must be set!");
    info!("Connecting to message broker...");

    let conn = match Connection::connect(&addr, ConnectionProperties::default()).await {
        Ok(conn) => {
            info!("Connected to message broker");
            conn
        }
        Err(e) => {
            panic!("Error connecting to message broker: {}", e);
        }
    };
    match conn.create_channel().await {
        Ok(channel) => channel,
        Err(e) => {
            panic!("Error creating channel: {}", e);
        }
    }
}
