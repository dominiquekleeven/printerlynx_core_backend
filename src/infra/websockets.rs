use std::net::SocketAddr;

use axum::extract::ws::{Message, WebSocket};
use axum::extract::{ConnectInfo, WebSocketUpgrade};
use axum::response::IntoResponse;
use axum::{headers, TypedHeader};
use futures_util::stream::SplitSink;
use futures_util::{stream::StreamExt, SinkExt};
use serde::{Deserialize, Serialize};
use tracing::info;
use uuid::Uuid;

#[derive(Debug)]
#[allow(dead_code)]
pub struct WebSocketSession {
    pub uuid: String,
    pub addr: SocketAddr,
    pub authenticated: bool,
    pub sender: SplitSink<WebSocket, Message>,
}

#[derive(Serialize, Deserialize, Debug)]
#[allow(dead_code)]
struct WebSocketSessionInfo {
    pub uuid: String,
    pub addr: SocketAddr,
    pub authenticated: bool,
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
    let (sender, mut receiver) = socket.split();
    let mut session = WebSocketSession {
        uuid: Uuid::new_v4().to_string(),
        addr,
        sender,
        authenticated: false,
    };
    info!("Created session: {:?}", session);

    // convert session to json
    let session_info = WebSocketSessionInfo {
        uuid: session.uuid.clone(),
        addr: session.addr,
        authenticated: session.authenticated,
    };
    let session_json = serde_json::to_string(&session_info).expect("Failed to serialize session");

    // send session info to client
    let _ = session.sender.send(Message::from(session_json)).await;

    tokio::spawn(async move {
        while let Some(message) = &receiver.next().await {
            info!("Received message: {:?} from {:?}", message, addr);

            let response = "Pong";
            match session.sender.send(Message::from(response)).await {
                Ok(_) => info!("Sent message: {:?} to {:?}", response, addr),
                Err(e) => {
                    info!(
                        "Error sending message: {:?} to {:?}: {:?}",
                        response, addr, e
                    );
                    break;
                }
            }
        }
        info!("Connection closed with {:?}", addr);
    });
}
