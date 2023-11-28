use serde::{Deserialize, Serialize};

#[allow(dead_code)]
#[derive(Debug, Serialize, Deserialize)]
pub struct AgentService {
    pub uuid: String,
    pub token: String,
    pub name: String,
    pub state: AgentServiceState,
}

#[allow(dead_code)]
#[derive(Debug, Serialize, Deserialize)]
pub struct AgentServiceState {
    pub connected: bool,
    pub registered: bool,
}

#[allow(dead_code)]
#[derive(Debug, Serialize, Deserialize)]
pub struct AgentServiceRegistrationRequest {
    pub token: String,
}

#[allow(dead_code)]
#[derive(Debug, Serialize, Deserialize)]
pub struct AgentServiceRegistrationResponse {
    pub broker_user: String,
    pub broker_password: String,
}
