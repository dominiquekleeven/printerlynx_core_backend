use std::sync::Arc;

use crate::common::app_error::AppError;
use crate::common::mapper::account_to_viewmodel;
use axum::extract::State;
use axum::routing::get;
use axum::{middleware, Extension, Json, Router};
use tracing::info;

use crate::middlewares::auth_middleware;
use crate::models::account::model::AccountViewModel;
use crate::services::user_service::{UserService, UserServiceImpl};
use crate::AppState;

/// Initializes the user controller, defining the routes and middlewares
pub fn init() -> Router<Arc<AppState>> {
    info!("Ok");
    Router::new()
        .route("/users/info", get(info))
        .route_layer(middleware::from_fn(auth_middleware::handle))
}

/// /api/v1/users/info (protected) - Retrieves the user info based on the bearer token
pub async fn info(
    State(state): State<Arc<AppState>>,
    Extension(user_uuid): Extension<String>,
) -> Result<Json<AccountViewModel>, AppError> {
    let user_service = UserServiceImpl::new(state.pool.clone());

    let account = match user_service.get_by_uuid(&user_uuid).await {
        Ok(account) => account,
        Err(err) => return Err(err),
    };

    let viewmodel = account_to_viewmodel(account);

    Ok(Json(viewmodel))
}
