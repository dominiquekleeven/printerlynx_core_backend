use std::net::SocketAddr;
use std::sync::Arc;

use axum::extract::ws::{Message, WebSocket};
use axum::extract::{ConnectInfo, WebSocketUpgrade};
use axum::response::IntoResponse;
use futures_util::{stream::StreamExt, SinkExt};
use serde::{Deserialize, Serialize};
use tracing::info;

#[derive(Serialize, Deserialize, Debug)]
struct UserWebSocketSession {
    pub user_uuid: String,
    pub authenticated: bool,
}

struct WebSocketState {
    pub user_web_socket_session: Arc<UserWebSocketSession>,
}

pub async fn handler(
    ws: WebSocketUpgrade,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
) -> impl IntoResponse {
    info!("User at {addr} connected.");
    ws.on_upgrade(move |socket| handle_socket(socket, addr))
}

async fn handle_socket(socket: WebSocket, addr: SocketAddr) {
    let (mut sender, mut receiver) = socket.split();
    let session = UserWebSocketSession {
        authenticated: false,
        user_uuid: "".to_string(),
    };

    let ws_state = Arc::new(WebSocketState {
        user_web_socket_session: Arc::new(session),
    });

    info!("Created session: {:?}", ws_state.user_web_socket_session);
    let session_json = serde_json::to_string(&ws_state.user_web_socket_session)
        .expect("Failed to serialize session");

    // send session info to client
    let _ = sender.send(Message::from(session_json)).await;

    // spawn receiver task
    tokio::spawn(async move {
        while let Some(message) = &receiver.next().await {
            info!("Received message: {:?} from {:?}", message, addr);
        }
        info!("Connection closed with {:?}", addr);
    });
}
