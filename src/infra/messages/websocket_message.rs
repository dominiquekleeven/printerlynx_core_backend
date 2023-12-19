use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct WebSocketMessage {
    pub body: String,
    pub message_type: WebSocketMessageType,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub enum WebSocketMessageType {
    UserAuthentication,
    User,
    AgentAuthentication,
    Agent,
    Printer,
    Error,
}
