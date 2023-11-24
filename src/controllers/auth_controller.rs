use std::sync::Arc;

use crate::AppState;
use axum::extract::State;
use axum::routing::post;
use axum::{Json, Router};
use tracing::info;

use crate::common::app_error::AppError;
use crate::common::jwt_token::JwtToken;
use crate::models::account_model::{AccountLoginModel, AccountRegisterModel};
use crate::services::auth_service::{AuthService, AuthServiceImpl};

/// Initializes the auth controller, defining the routes and middlewares
pub fn init() -> Router<Arc<AppState>> {
    info!("Ok");
    Router::new()
        .route("/auth/login", post(login))
        .route("/auth/register", post(register))
}

pub async fn register(
    State(state): State<Arc<AppState>>,
    Json(json): Json<AccountRegisterModel>,
) -> Result<Json<JwtToken>, AppError> {
    let account_service = AuthServiceImpl::new(state.pool.clone());

    match account_service.register(json).await {
        Ok(token) => Ok(Json(token)),
        Err(err) => Err(err),
    }
}

pub async fn login(
    State(state): State<Arc<AppState>>,
    Json(json): Json<AccountLoginModel>,
) -> Result<Json<JwtToken>, AppError> {
    let account_service = AuthServiceImpl::new(state.pool.clone());

    match account_service.login(json).await {
        Ok(token) => Ok(Json(token)),
        Err(err) => Err(err),
    }
}
