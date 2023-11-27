use std::net::SocketAddr;

use axum::{headers, TypedHeader};
use axum::extract::{ConnectInfo, WebSocketUpgrade};
use axum::extract::ws::{Message, WebSocket};
use axum::response::IntoResponse;
use futures_util::{
    SinkExt,
    stream::StreamExt,
};
use tracing::info;
use uuid::Uuid;

#[derive(Debug)]
#[allow(dead_code)]
struct WebSocketSession {
    uuid: String,
    addr: SocketAddr,
}

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
    let (mut sender, mut receiver) = socket.split();
    let session = WebSocketSession {
        uuid: Uuid::new_v4().to_string(),
        addr,
    };
    info!("Created session: {:?}", session);

    while let Some(message) = receiver.next().await {
        info!("Received message: {:?} from {:?}", message, addr);

        let response = "Pong";
        match sender.send(Message::from(response)).await {
            Ok(_) => info!("Sent message: {:?} to {:?}", response, addr),
            Err(e) => {
                info!("Error sending message: {:?} to {:?}: {:?}", response, addr, e);
                break;
            }
        }
    }
    info!("Connection closed with {:?}", addr);
}



