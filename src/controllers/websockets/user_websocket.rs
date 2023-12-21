use std::net::SocketAddr;
use std::sync::Arc;

use axum::Error;
use axum::extract::{ConnectInfo, State, WebSocketUpgrade};
use axum::extract::ws::{Message, WebSocket};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use futures_util::{SinkExt, stream::StreamExt};
use serde::{Deserialize, Serialize};
use tokio::sync::Mutex;
use tracing::{info, warn};

use crate::AppState;
use crate::common::app_error::AppError;
use crate::common::jwt_token::decode_token;
use crate::controllers::websockets::websocket_message::{WebSocketMessage, WebSocketMessageType};

#[derive(Serialize, Deserialize, Debug)]
pub struct UserSession {
    pub uuid: String,
    pub authenticated: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UserWebSocketSession {
    pub user: UserSession,
}


pub async fn handler(
    ws: WebSocketUpgrade,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    State(state): State<Arc<AppState>>,
) -> impl IntoResponse {
    info!("User at {addr} connected.");
    ws.on_upgrade(move |socket| handle_socket(socket, addr, State(state)))
}

async fn handle_socket(socket: WebSocket, addr: SocketAddr, State(_state): State<Arc<AppState>>) {
    let (mut sender, mut receiver) = socket.split();

    let session = Arc::new(Mutex::new(UserWebSocketSession {
        user: UserSession {
            uuid: "".to_string(),
            authenticated: false,
        }
    }));

    info!("Created session: {:?}", session);

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
            if message.message_type == WebSocketMessageType::UserAuthentication {
                match handle_auth_message(message, &session).await {
                    Ok(_) => {
                        let session_info = session.lock().await;
                        let session_json = serde_json::to_string(&session_info.user).unwrap();
                        let message = WebSocketMessage {
                            message_type: WebSocketMessageType::UserAuthentication,
                            body: session_json,
                        };
                        let message = serde_json::to_string(&message).unwrap();
                        let _ = sender.send(Message::from(message)).await;

                        continue;
                    }
                    Err(err) => {
                        warn!("Error authenticating: {:?}", err);
                        let message = WebSocketMessage {
                            message_type: WebSocketMessageType::Error,
                            body: err.to_string(),
                        };
                        let _ = sender.send(Message::from(serde_json::to_string(&message).unwrap())).await;
                        continue;
                    }
                }
            }

            //if session is not authenticated, ignore message
            if !session.lock().await.user.authenticated {
                let message = WebSocketMessage {
                    message_type: WebSocketMessageType::Error,
                    body: "Session not authenticated".to_string(),
                };
                let _ = sender.send(Message::from(serde_json::to_string(&message).unwrap())).await;
                continue;
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
    let message: WebSocketMessage = match serde_json::from_str(message) {
        Ok(message) => message,
        Err(_) => return Err(AppError::InternalServer),
    };

    Ok(message)
}

async fn handle_auth_message(message: WebSocketMessage, session: &Arc<Mutex<UserWebSocketSession>>) -> Result<(), AppError> {
    let token = message.body;
    let mut session = session.lock().await;

    if session.user.authenticated
    {
        return Ok(());
    }

    // check if token is valid and set session to authenticated
    match decode_token(&token) {
        Ok(jwt) => {
            session.user.authenticated = true;
            session.user.uuid = jwt.claims.sub;
        },
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
