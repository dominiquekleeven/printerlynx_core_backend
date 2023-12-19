use crate::common::app_error::AppError;
use crate::middlewares::auth_middleware;
use crate::models::agent::{AgentAddRequest, AgentViewModel};
use crate::models::view_model::ViewModel;
use crate::services::agent_service::{AgentService, AgentServiceImpl};
use crate::AppState;
use axum::extract::{Path, State};
use axum::routing::{delete, get, post};
use axum::{middleware, Extension, Json, Router};
use std::sync::Arc;
use tracing::info;

pub fn init() -> Router<Arc<AppState>> {
    info!("Ok");
    Router::new()
        .route("/agents", get(get_all))
        .route("/agents", post(add))
        .route("/agents/:uuid", delete(delete_by_uuid))
        .route_layer(middleware::from_fn(auth_middleware::handle))
}

async fn add(
    State(state): State<Arc<AppState>>,
    Extension(user_uuid): Extension<String>,
    Json(json): Json<AgentAddRequest>,
) -> Result<Json<AgentViewModel>, AppError> {
    let agent_service = AgentServiceImpl::new(state.db_pool.clone());
    let agent = agent_service.add(&user_uuid, json).await?;

    let viewmodel = agent.to_viewmodel();
    Ok(Json(viewmodel))
}

async fn get_all(
    State(state): State<Arc<AppState>>,
    Extension(user_uuid): Extension<String>,
) -> Result<Json<Vec<AgentViewModel>>, AppError> {
    let agent_service = AgentServiceImpl::new(state.db_pool.clone());
    let agents = agent_service.get_all(&user_uuid).await?;

    let agents = agents
        .into_iter()
        .map(|agent| agent.to_viewmodel())
        .collect::<Vec<AgentViewModel>>();

    Ok(Json(agents))
}

async fn delete_by_uuid(
    State(state): State<Arc<AppState>>,
    Extension(user_uuid): Extension<String>,
    Path(uuid): Path<String>,
) -> Result<Json<bool>, AppError> {
    let agent_service = AgentServiceImpl::new(state.db_pool.clone());
    agent_service.delete(&user_uuid, &uuid).await?;

    Ok(Json(true))
}
