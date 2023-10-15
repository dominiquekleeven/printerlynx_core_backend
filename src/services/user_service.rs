use std::sync::Arc;

use axum::async_trait;
use axum::http::StatusCode;
use sea_query::{Expr, MysqlQueryBuilder, Query};
use sqlx::{FromRow, MySql, MySqlPool, Pool};

use crate::common::app_error::AppError;
use crate::models::account_model::{Account, AccountDbModel};

#[async_trait]
pub trait UserService {
    async fn get_by_uuid(&self, uuid: &str) -> Result<AccountDbModel, AppError>;
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
