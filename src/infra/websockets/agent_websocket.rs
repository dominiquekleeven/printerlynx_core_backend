use std::net::SocketAddr;

use axum::{headers, TypedHeader};
use axum::extract::{ConnectInfo, WebSocketUpgrade};
use axum::extract::ws::{Message, WebSocket};
use axum::response::IntoResponse;
use futures_util::{SinkExt, stream::StreamExt};
use futures_util::stream::SplitSink;
use serde::{Deserialize, Serialize};
use tracing::info;
use uuid::Uuid;

pub async fn handler(
    ws: WebSocketUpgrade,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
) -> impl IntoResponse {
    info!("agent at {addr} connected.");
    ws.on_upgrade(move |socket| handle_socket(socket, addr))
}

async fn handle_socket(socket: WebSocket, addr: SocketAddr) {
    let (_sender, mut receiver) = socket.split();


    tokio::spawn(async move {
        while let Some(message) = &receiver.next().await {
            info!("Received message: {:?} from {:?}", message, addr);
        }
        info!("Connection closed with {:?}", addr);
    });
}
