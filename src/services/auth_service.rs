use std::sync::Arc;

use axum::async_trait;
use axum::http::StatusCode;
use chrono::Utc;
use password_auth::{generate_hash, verify_password};
use sea_query::{Expr, MysqlQueryBuilder, Query};
use sqlx::{Executor, FromRow, MySql, MySqlPool, Pool};
use tracing::error;
use uuid::Uuid;

use crate::common::app_error::AppError;
use crate::common::jwt_token::{generate_token, Claims, JwtToken};
use crate::models::account::model::{
    Account, AccountDbModel, AccountLoginModel, AccountRegisterModel,
};

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

        if register.password.len() < 6 {
            return Err(AppError::Register {
                message: "Password must be at least 6 characters long".to_string(),
                status: StatusCode::BAD_REQUEST,
            });
        }

        check_if_username_exists(self.pool.clone(), &register.username).await?;
        let uuid = Uuid::new_v4().to_string();
        let account = AccountDbModel {
            uuid,
            username: register.username,
            password: generate_hash(register.password),
            created_at: Utc::now().timestamp().to_string(),
            updated_at: Utc::now().timestamp().to_string(),
        };

        insert_user(self.pool.clone(), &account).await?;

        let token = generate_token(create_claims(&account.uuid));
        Ok(JwtToken { token })
    }

    /// Login a user and return a JWT token
    async fn login(&self, login: AccountLoginModel) -> Result<JwtToken, AppError> {
        let account = get_user_by_username(self.pool.clone(), &login.username).await?;

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

/// Insert a new user into the database using the account model struct
async fn insert_user(
    pool: Arc<MySqlPool>,
    account_model: &AccountDbModel,
) -> Result<bool, AppError> {
    let sql = Query::insert()
        .into_table(Account::Table)
        .columns([
            Account::Uuid,
            Account::Username,
            Account::Password,
            Account::CreatedAt,
            Account::UpdatedAt,
        ])
        .values_panic([
            account_model.uuid.to_string().into(),
            account_model.username.to_string().into(),
            account_model.password.to_string().into(),
            account_model.created_at.to_string().into(),
            account_model.updated_at.to_string().into(),
        ])
        .to_string(MysqlQueryBuilder)
        .to_owned();

    let mut conn = pool.acquire().await.unwrap();

    match conn.execute(&*sql).await {
        Ok(_) => Ok(true),
        Err(e) => {
            error!("Error creating account: {}", e);
            Err(AppError::InternalServer {
                message: "Error creating account".to_string(),
            })
        }
    }
}

/// Select a user by their username from the database
async fn get_user_by_username(
    pool: Arc<MySqlPool>,
    username: &str,
) -> Result<AccountDbModel, AppError> {
    let sql = Query::select()
        .columns([
            Account::Uuid,
            Account::Username,
            Account::Password,
            Account::CreatedAt,
            Account::UpdatedAt,
        ])
        .from(Account::Table)
        .and_where(Expr::col(Account::Username).eq(username))
        .to_string(MysqlQueryBuilder);

    let row = sqlx::query(&sql).fetch_optional(&*pool).await.unwrap();

    if row.is_none() {
        return Err(AppError::Login {
            message: "Invalid username or password".to_string(),
            status: StatusCode::BAD_REQUEST,
        });
    }

    let row = row.expect("Error unwrapping row");
    let account = AccountDbModel::from_row(&row).expect("Error converting row to AccountModel");

    Ok(account)
}

/// Check if a username already exists in the database
async fn check_if_username_exists(pool: Arc<MySqlPool>, username: &str) -> Result<bool, AppError> {
    let sql = Query::select()
        .columns([Account::Uuid])
        .from(Account::Table)
        .and_where(Expr::col(Account::Username).eq(username))
        .to_string(MysqlQueryBuilder);

    let row = sqlx::query(&sql).fetch_optional(&*pool).await.unwrap();

    if row.is_some() {
        return Err(AppError::Register {
            message: "Username already exists".to_string(),
            status: StatusCode::BAD_REQUEST,
        });
    }

    Ok(true)
}
