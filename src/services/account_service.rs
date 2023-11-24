use std::sync::Arc;

use axum::async_trait;
use axum::http::StatusCode;
use sea_query::{Expr, MysqlQueryBuilder, Query};
use sqlx::{Executor, FromRow, MySql, Pool};
use tracing::error;

use crate::common::app_error::AppError;
use crate::models::account_model::{Account, AccountDbModel};

#[async_trait]
pub trait AccountService {
    async fn get_by_uuid(&self, uuid: &str) -> Result<AccountDbModel, AppError>;
    async fn insert(&self, account: &AccountDbModel) -> Result<bool, AppError>;
    async fn get_by_username(&self, username: &str) -> Result<AccountDbModel, AppError>;
}

pub struct AccountServiceImpl {
    pool: Arc<Pool<MySql>>,
}

impl AccountServiceImpl {
    pub fn new(pool: Arc<Pool<MySql>>) -> Self {
        AccountServiceImpl { pool }
    }
}

#[async_trait]
impl AccountService for AccountServiceImpl {
    /// Retrieves the user info based on their uuid
    async fn get_by_uuid(&self, uuid: &str) -> Result<AccountDbModel, AppError> {
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

        let pool = self.pool.clone();
        let row = sqlx::query(&sql).fetch_optional(&*pool).await.unwrap();

        if row.is_none() {
            return Err(AppError::User {
                message: "No user found".to_string(),
                status: StatusCode::NOT_FOUND,
            });
        }

        let row = row.expect("Error unwrapping row");
        let account =
            AccountDbModel::from_row(&row).expect("Error converting row to AccountDbModel");

        Ok(account)
    }

    async fn insert(&self, account: &AccountDbModel) -> Result<bool, AppError> {
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
                account.uuid.to_string().into(),
                account.username.to_string().into(),
                account.email.to_string().into(),
                account.password.to_string().into(),
                account.created_at.to_string().into(),
                account.updated_at.to_string().into(),
            ])
            .to_string(MysqlQueryBuilder)
            .to_owned();

        if !is_username_unique(self.pool.clone(), &account.username).await? {
            return Err(AppError::User {
                message: "Username is already taken".to_string(),
                status: StatusCode::BAD_REQUEST,
            });
        }

        let pool = self.pool.clone();
        let mut conn = pool.acquire().await.unwrap();

        match conn.execute(&*sql).await {
            Ok(_) => Ok(true),
            Err(e) => {
                error!("Error creating account: {}", e);
                Err(AppError::InternalServer)
            }
        }
    }

    /// Select a user by their username from the database
    async fn get_by_username(&self, username: &str) -> Result<AccountDbModel, AppError> {
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

        let pool = self.pool.clone();
        let row = sqlx::query(&sql).fetch_optional(&*pool).await.unwrap();

        if row.is_none() {
            return Err(AppError::User {
                message: "User not found".to_string(),
                status: StatusCode::NOT_FOUND,
            });
        }

        let row = row.expect("Error unwrapping row");
        let account = AccountDbModel::from_row(&row).expect("Error converting row to AccountModel");

        Ok(account)
    }
}

async fn is_username_unique(pool: Arc<Pool<MySql>>, username: &str) -> Result<bool, AppError> {
    let sql = Query::select()
        .columns([Account::Uuid])
        .from(Account::Table)
        .and_where(Expr::col(Account::Username).eq(username))
        .to_string(MysqlQueryBuilder);

    let row = match sqlx::query(&sql).fetch_optional(&*pool).await {
        Ok(row) => row,
        Err(e) => {
            error!("Error checking if username is unique: {}", e);
            return Err(AppError::InternalServer);
        }
    };

    if row.is_some() {
        return Ok(false);
    }
    Ok(true)
}
