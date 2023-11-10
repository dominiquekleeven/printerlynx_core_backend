use std::sync::Arc;

use axum::async_trait;
use axum::http::StatusCode;
use chrono::Utc;
use password_auth::{generate_hash, verify_password};
use sqlx::{MySql, Pool};
use uuid::Uuid;

use crate::common::app_error::AppError;
use crate::common::jwt_token::{generate_token, Claims, JwtToken};
use crate::models::account_model::{AccountDbModel, AccountLoginModel, AccountRegisterModel};
use crate::services::user_service::{UserService, UserServiceImpl};

#[async_trait]
pub trait AuthService {
    async fn register(&self, account: AccountRegisterModel) -> Result<JwtToken, AppError>;
    async fn login(&self, account: AccountLoginModel) -> Result<JwtToken, AppError>;
}

pub struct AuthServiceImpl {
    pool: Arc<Pool<MySql>>,
}

impl AuthServiceImpl {
    pub fn new(pool: Arc<Pool<MySql>>) -> Self {
        AuthServiceImpl { pool }
    }
}

#[async_trait]
impl AuthService for AuthServiceImpl {
    /// Register a new user and return a JWT token
    async fn register(&self, register: AccountRegisterModel) -> Result<JwtToken, AppError> {
        if register.password != register.password_confirmation {
            return Err(AppError::Register {
                message: "Password and password confirmation do not match".to_string(),
                status: StatusCode::BAD_REQUEST,
            });
        }

        if register.username.len() < 3 {
            return Err(AppError::Register {
                message: "Username must be at least 3 characters long".to_string(),
                status: StatusCode::BAD_REQUEST,
            });
        }

        if !register.email.contains('@') {
            return Err(AppError::Register {
                message: "Email is not valid".to_string(),
                status: StatusCode::BAD_REQUEST,
            });
        }

        if register.password.len() < 6 {
            return Err(AppError::Register {
                message: "Password must be at least 6 characters long".to_string(),
                status: StatusCode::BAD_REQUEST,
            });
        }

        let user_service = UserServiceImpl::new(self.pool.clone());
        user_service
            .check_if_username_exists(&register.username)
            .await?;

        let uuid = Uuid::new_v4().to_string();
        let account = AccountDbModel {
            uuid,
            username: register.username,
            email: register.email,
            password: generate_hash(register.password),
            created_at: Utc::now().timestamp().to_string(),
            updated_at: Utc::now().timestamp().to_string(),
        };

        if (user_service.insert(&account).await).is_err() {
            return Err(AppError::Register {
                message: "Failed to create account, please try again".to_string(),
                status: StatusCode::INTERNAL_SERVER_ERROR,
            });
        }

        let token = generate_token(create_claims(&account.uuid));
        Ok(JwtToken { token })
    }

    /// Login a user and return a JWT token
    async fn login(&self, login: AccountLoginModel) -> Result<JwtToken, AppError> {
        let user_service = UserServiceImpl::new(self.pool.clone());

        let account = match user_service.get_by_username(&login.username).await {
            Ok(account) => account,
            Err(_) => {
                return Err(AppError::Login {
                    message: "Invalid username or password".to_string(),
                    status: StatusCode::BAD_REQUEST,
                });
            }
        };

        match verify_password(&login.password, &account.password) {
            Ok(_) => {} // Do nothing
            Err(_) => {
                return Err(AppError::Login {
                    message: "Invalid username or password".to_string(),
                    status: StatusCode::BAD_REQUEST,
                });
            }
        }

        let token = generate_token(create_claims(&account.uuid));
        Ok(JwtToken { token })
    }
}

/// Create a new claims struct for the JWT token
fn create_claims(uuid: &str) -> Claims {
    Claims {
        exp: Utc::now().timestamp() as usize + 31536000,
        iss: "Printerlynx".to_string(),
        sub: uuid.to_string(),
    }
}
