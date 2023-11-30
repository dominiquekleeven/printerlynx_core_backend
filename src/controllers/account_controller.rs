use std::sync::Arc;

use crate::common::app_error::AppError;
use axum::extract::State;
use axum::routing::get;
use axum::{middleware, Extension, Json, Router};
use tracing::info;

use crate::middlewares::auth_middleware;
use crate::models::account::AccountViewModel;
use crate::models::view_model::ViewModel;
use crate::services::account_service::{AccountService, AccountServiceImpl};
use crate::AppState;

/// Initializes the user controller, defining the routes and middlewares
pub fn init() -> Router<Arc<AppState>> {
    info!("Ok");
    Router::new()
        .route("/accounts/me", get(info))
        .route_layer(middleware::from_fn(auth_middleware::handle))
}

pub async fn info(
    State(state): State<Arc<AppState>>,
    Extension(user_uuid): Extension<String>,
) -> Result<Json<AccountViewModel>, AppError> {
    let account_service = AccountServiceImpl::new(state.db_pool.clone());

    let account = match account_service.get_by_uuid(&user_uuid).await {
        Ok(account) => account,
        Err(err) => return Err(err),
    };

    let viewmodel = account.to_viewmodel();
    Ok(Json(viewmodel))
}
