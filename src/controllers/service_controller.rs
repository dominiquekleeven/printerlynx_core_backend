use std::sync::Arc;
use crate::common::app_error::AppError;
use crate::models::agent::agent_service::{
    AgentServiceRegistrationRequest, AgentServiceRegistrationResponse,
};
use crate::AppState;
use axum::extract::State;
use axum::routing::post;
use axum::{Json, Router};
use tracing::info;

pub fn init() -> Router<Arc<AppState>> {
    info!("Ok");
    Router::new()
        .route("/services/agent/register", post(agent_register))
}

pub async fn agent_register(
    State(state): State<Arc<AppState>>,
    Json(json): Json<AgentServiceRegistrationRequest>,
) -> Result<Json<AgentServiceRegistrationResponse>, AppError> {

    let response = AgentServiceRegistrationResponse {
        broker_user: "".to_string(),
        broker_password: "".to_string(),
    };

    Ok(Json(response))
}
