use std::net::SocketAddr;
use std::sync::Arc;

use axum::Error;
use axum::extract::{ConnectInfo, State, WebSocketUpgrade};
use axum::extract::ws::{Message, WebSocket};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use futures_util::{SinkExt, stream::StreamExt};
use serde::{Deserialize, Serialize};
use tracing::{info, warn};

use crate::AppState;
use crate::common::app_error::AppError;
use crate::common::jwt_token::decode_token;
use crate::infra::messages::websocket_message::{WebSocketMessage, WebSocketMessageType};

#[derive(Serialize, Deserialize, Debug)]
pub struct UserWebSocketSession {
    pub user_uuid: String,
    pub authenticated: bool,
}

struct WebSocketState {
    pub user_web_socket_session: Arc<UserWebSocketSession>,
}

pub async fn handler(
    ws: WebSocketUpgrade,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    State(state): State<Arc<AppState>>,
) -> impl IntoResponse {
    info!("User at {addr} connected.");
    ws.on_upgrade(move |socket| handle_socket(socket, addr, State(state)))
}

async fn handle_socket(socket: WebSocket, addr: SocketAddr, State(state): State<Arc<AppState>>) {
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
            info!("Received message: {:?} from {:?}", &message, addr);

            let message = match parse_message(message) {
                Ok(message) => message,
                Err(err) => {
                    warn!("Error parsing message: {:?}", err);
                    let _ = sender.send(Message::from("Error parsing message")).await;
                    continue;
                }
            };

            // check if message is authentication
            if message.message_type == WebSocketMessageType::Authentication {
                handle_auth_message(message).expect("TODO: panic message");
            }
        }
        info!("Connection closed with {:?}", addr);
    });
}


fn parse_message(message: &Result<Message, Error>) -> Result<WebSocketMessage, AppError> {
    // check if is valid
    let message = match message {
        Ok(message) => message,
        Err(_) => return Err(AppError::InternalServer),
    };
    // check if message is text
    let message = match message.to_text() {
        Ok(message) => message,
        Err(_) => return Err(AppError::InternalServer),
    };

    // try to deserialize message
    let message: WebSocketMessage = match serde_json::from_str(&message) {
        Ok(message) => message,
        Err(_) => return Err(AppError::InternalServer),
    };

    Ok(message)
}


fn handle_auth_message(message: WebSocketMessage) -> Result<(), AppError> {
    let token = message.token;

    // check if token is valid
    let token = match decode_token(&token) {
        Ok(token) => token,
        Err(_) => {
            warn!("Invalid token");
            return Err(AppError::Token {
                message: "Invalid token".to_string(),
                status: StatusCode::UNAUTHORIZED,
            });
        }
    };

    Ok(())
}
