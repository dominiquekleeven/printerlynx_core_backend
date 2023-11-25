use std::sync::Arc;

use crate::AppState;
use axum::http::header::{AUTHORIZATION, CONTENT_TYPE, USER_AGENT};
use axum::http::Method;
use axum::routing::get;
use axum::Router;
use tower_http::cors::{Any, CorsLayer};
use tower_http::trace;
use tracing::Level;

use crate::controllers::{account_controller, agent_controller};
use crate::controllers::{auth_controller, printfile_controller};

pub async fn create(state: Arc<AppState>) -> Router {
    let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST, Method::DELETE, Method::PUT])
        .allow_origin(Any)
        .allow_headers([AUTHORIZATION, USER_AGENT, CONTENT_TYPE]);

    let trace_layer = tower_http::trace::TraceLayer::new_for_http()
        .make_span_with(trace::DefaultMakeSpan::new().level(Level::INFO))
        .on_response(trace::DefaultOnResponse::new().level(Level::INFO));

    let auth_endpoints = auth_controller::init();
    let account_endpoints = account_controller::init();
    let printfile_endpoints = printfile_controller::init();
    let agent_endpoints = agent_controller::init();

    Router::new()
        .route("/health", get(|| async { "OK" }))
        .nest("/api/v1", auth_endpoints)
        .nest("/api/v1", account_endpoints)
        .nest("/api/v1", printfile_endpoints)
        .nest("/api/v1", agent_endpoints)
        .layer(cors)
        .layer(trace_layer)
        .with_state(state)
}
