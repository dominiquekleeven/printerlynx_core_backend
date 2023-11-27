use axum::extract::ws::{Message, WebSocket};
use axum::extract::{ConnectInfo, WebSocketUpgrade};

use axum::response::IntoResponse;
use axum::{headers, TypedHeader};
use futures_util::{
    stream::{SplitSink, SplitStream, StreamExt},
    SinkExt,
};
use std::net::SocketAddr;
use tracing::info;

pub async fn handler(
    ws: WebSocketUpgrade,
    user_agent: Option<TypedHeader<headers::UserAgent>>,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
) -> impl IntoResponse {
    let user_agent = if let Some(TypedHeader(user_agent)) = user_agent {
        user_agent.to_string()
    } else {
        String::from("Unknown")
    };
    info!("{user_agent} at {addr} connected.");
    ws.on_upgrade(move |socket| handle_socket(socket, addr))
}

async fn handle_socket(socket: WebSocket, addr: SocketAddr) {
    let (sender, receiver) = socket.split();
    info!("New connection from: {}", addr);

    tokio::spawn(write(sender));
    tokio::spawn(read(receiver));
}

async fn read(mut receiver: SplitStream<WebSocket>) {
    let message = receiver.next().await;
    info!("Received message: {:?}", message);
}

async fn write(mut sender: SplitSink<WebSocket, Message>) {
    sender
        .send(Message::Text("Connection accepted.".parse().unwrap()))
        .await
        .unwrap();
}
