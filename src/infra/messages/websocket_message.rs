use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct WebSocketMessage {
    pub token: String,
    pub message_type: WebSocketMessageType,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub enum WebSocketMessageType {
    Authentication,
    User,
    AgentAuthentication,
    Agent,
}