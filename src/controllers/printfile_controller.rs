use std::sync::Arc;

use axum::extract::Path;
use axum::extract::{DefaultBodyLimit, Multipart, State};
use axum::routing::{delete, get, post};
use axum::{middleware, Extension, Json, Router};
use tracing::info;

use crate::common::app_error::AppError;
use crate::middlewares::auth_middleware;
use crate::models::printfile_model::PrintFileViewModel;
use crate::services::printfile_service::{PrintFileService, PrintFileServiceImpl};
use crate::AppState;

pub fn init() -> Router<Arc<AppState>> {
    info!("Ok");
    Router::new()
        .route("/files", get(get_all))
        .route("/files/:uuid", get(get_by_uuid))
        .route("/files/upload", post(upload))
        .route("/files/:uuid", delete(delete_by_uuid))
        .route_layer(DefaultBodyLimit::max(1024 * 1024 * 20)) // 20MB
        .route_layer(middleware::from_fn(auth_middleware::handle))
}

async fn get_all(
    State(state): State<Arc<AppState>>,
    Extension(user_uuid): Extension<String>,
) -> Result<Json<Vec<PrintFileViewModel>>, AppError> {
    let printfile_service = PrintFileServiceImpl::new(state.pool.clone());

    let printfiles = printfile_service.get_all(&user_uuid).await?;

    let files = printfiles
        .into_iter()
        .map(|printfile| printfile.to_viewmodel())
        .collect::<Vec<PrintFileViewModel>>();

    Ok(Json(files))
}

async fn get_by_uuid(
    State(state): State<Arc<AppState>>,
    Extension(user_uuid): Extension<String>,
    Path(uuid): Path<String>,
) -> Result<Json<PrintFileViewModel>, AppError> {
    let printfile_service = PrintFileServiceImpl::new(state.pool.clone());

    let printfile = printfile_service.get_by_uuid(&user_uuid, &uuid).await?;
    let printfile = printfile.to_viewmodel();

    Ok(Json(printfile))
}

async fn upload(
    State(state): State<Arc<AppState>>,
    Extension(user_uuid): Extension<String>,
    multipart: Multipart,
) -> Result<Json<PrintFileViewModel>, AppError> {
    let printfile_service = PrintFileServiceImpl::new(state.pool.clone());
    let printfile = printfile_service.upload(&user_uuid, multipart).await?;
    let printfile = printfile.to_viewmodel();

    Ok(Json(printfile))
}

async fn delete_by_uuid(
    State(state): State<Arc<AppState>>,
    Extension(user_uuid): Extension<String>,
    Path(uuid): Path<String>,
) -> Result<Json<bool>, AppError> {
    let printfile_service = PrintFileServiceImpl::new(state.pool.clone());
    let deleted = printfile_service.delete(&user_uuid, &uuid).await?;
    Ok(Json(deleted))
}
