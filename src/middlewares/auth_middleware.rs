use axum::headers::authorization::Bearer;
use axum::headers::Authorization;
use axum::http::StatusCode;
use axum::{http::Request, middleware::Next, response::Response, TypedHeader};
use tracing::{info, warn};

use crate::common::app_error::AppError;
use crate::common::jwt_token::decode_token;

/// Responsible for handling the authentication of the requests
/// - If the token is valid, the request will be processed, otherwise it will return a 401 Unauthorized
/// - Processed user uuid will be stored in the request extensions
pub async fn handle<B>(
    TypedHeader(auth): TypedHeader<Authorization<Bearer>>,
    mut request: Request<B>,
    next: Next<B>,
) -> Result<Response, AppError> {
    let time = std::time::Instant::now();
    let token = auth.token().to_string();
    let jwt = match decode_token(&token) {
        Ok(jwt) => jwt,
        Err(_) => {
            warn!("Invalid token");
            return Err(AppError::Token {
                message: "Invalid token".to_string(),
                status: StatusCode::UNAUTHORIZED,
            });
        }
    };

    info!(
        "finished processing protected request (duration: {}μs)",
        time.elapsed().as_micros()
    );
    let user_uuid = jwt.claims.sub;
    request.extensions_mut().insert(user_uuid);
    let response = next.run(request).await;
    Ok(response)
}
