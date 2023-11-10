use std::sync::Arc;

use axum::async_trait;
use axum::http::StatusCode;
use sea_query::{Expr, MysqlQueryBuilder, Query};
use sqlx::{Executor, FromRow, MySql, MySqlPool, Pool};
use tracing::error;

use crate::common::app_error::AppError;
use crate::models::account_model::{Account, AccountDbModel};

#[async_trait]
pub trait UserService {
    async fn get_by_uuid(&self, uuid: &str) -> Result<AccountDbModel, AppError>;
    async fn insert(&self, account: &AccountDbModel) -> Result<bool, AppError>;
    async fn get_by_username(&self, username: &str) -> Result<AccountDbModel, AppError>;
    async fn check_if_username_exists(&self, username: &str) -> Result<bool, AppError>;
}

pub struct UserServiceImpl {
    pool: Arc<Pool<MySql>>,
}

impl UserServiceImpl {
    pub fn new(pool: Arc<Pool<MySql>>) -> Self {
        UserServiceImpl { pool }
    }
}

#[async_trait]
impl UserService for UserServiceImpl {
    /// Retrieves the user info based on their uuid
    async fn get_by_uuid(&self, uuid: &str) -> Result<AccountDbModel, AppError> {
        let account = get_user_by_uuid(self.pool.clone(), uuid).await?;
        Ok(account)
    }

    async fn insert(&self, account: &AccountDbModel) -> Result<bool, AppError> {
        let result = insert_user(self.pool.clone(), account).await?;
        Ok(result)
    }

    async fn get_by_username(&self, username: &str) -> Result<AccountDbModel, AppError> {
        let account = get_user_by_username(self.pool.clone(), username).await?;
        Ok(account)
    }

    async fn check_if_username_exists(&self, username: &str) -> Result<bool, AppError> {
        let result = check_if_username_exists(self.pool.clone(), username).await?;
        Ok(result)
    }
}

/// Select a user by their uuid and return the account model
async fn get_user_by_uuid(pool: Arc<MySqlPool>, uuid: &str) -> Result<AccountDbModel, AppError> {
    let sql = Query::select()
        .columns([
            Account::Uuid,
            Account::Username,
            Account::Email,
            Account::Password,
            Account::CreatedAt,
            Account::UpdatedAt,
        ])
        .from(Account::Table)
        .and_where(Expr::col(Account::Uuid).eq(uuid))
        .to_string(MysqlQueryBuilder);

    let row = sqlx::query(&sql).fetch_optional(&*pool).await.unwrap();

    if row.is_none() {
        return Err(AppError::Token {
            message: "Invalid token, token has no associated user".to_string(),
            status: StatusCode::UNAUTHORIZED,
        });
    }

    let row = row.expect("Error unwrapping row");
    let account = AccountDbModel::from_row(&row).expect("Error converting row to AccountDbModel");

    Ok(account)
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
            Account::Email,
            Account::Password,
            Account::CreatedAt,
            Account::UpdatedAt,
        ])
        .values_panic([
            account_model.uuid.to_string().into(),
            account_model.username.to_string().into(),
            account_model.email.to_string().into(),
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
            Account::Email,
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
