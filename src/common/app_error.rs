use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Serialize, Deserialize)]
pub struct ErrorMessage {
    pub status: String,
    pub message: String,
}

/// AppError, used to handle errors in the application and return a proper response
/// - InternalServer: Generic error, used when the error is not known
/// - Register: Error related to the registration process
/// - Login: Error related to the login process
/// - Token: Error related to the token
/// - Validation: Error related to the validation of the request, string includes all the errors separated by a comma
#[allow(dead_code)]
#[derive(Error, Debug)]
pub enum AppError {
    #[error("{message:}")]
    InternalServer { message: String },

    #[error("{message:}")]
    Register { message: String, status: StatusCode },

    #[error("{message:}")]
    Login { message: String, status: StatusCode },

    #[error("{message:}")]
    Token { message: String, status: StatusCode },

    #[error("{messages:}")]
    Gcode {
        messages: String,
        status: StatusCode,
    },

    #[error("{message:}")]
    PrintFile { message: String, status: StatusCode },

    #[error("{messages:}")]
    Validation {
        messages: String,
        status: StatusCode,
    },
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let status = match self {
            AppError::InternalServer { .. } => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::Register { status, .. } => status,
            AppError::Login { status, .. } => status,
            AppError::Token { status, .. } => status,
            AppError::Gcode { status, .. } => status,
            AppError::PrintFile { status, .. } => status,
            AppError::Validation { status, .. } => status,
        };

        let json_body = Json(ErrorMessage {
            status: status.to_string(),
            message: self.to_string(),
        });

        (status, json_body).into_response()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_app_error_message() {
        let err = AppError::Register {
            message: "test".to_string(),
            status: StatusCode::BAD_REQUEST,
        };
        assert_eq!(err.to_string(), "test");
    }

    #[test]
    fn test_app_error_into_response() {
        let err = AppError::Register {
            message: "test".to_string(),
            status: StatusCode::BAD_REQUEST,
        };
        let response = err.into_response();
        assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    }
}
