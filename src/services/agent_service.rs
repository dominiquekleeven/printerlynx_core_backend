use std::sync::Arc;

use axum::async_trait;
use axum::http::StatusCode;
use chrono::Utc;
use sea_query::{Expr, MysqlQueryBuilder, Query};
use sqlx::{Executor, FromRow, MySql, Pool};
use tracing::error;
use uuid::Uuid;

use crate::common::app_error::AppError;
use crate::models::agent_model::{Agent, AgentAddRequest, AgentDbModel};

#[async_trait]
pub trait AgentService {
    async fn add(&self, user_uuid: &str, agent: AgentAddRequest) -> Result<AgentDbModel, AppError>;
    async fn delete(&self, user_uuid: &str, file_uuid: &str) -> Result<bool, AppError>;
    async fn get_all(&self, user_uuid: &str) -> Result<Vec<AgentDbModel>, AppError>;
}

pub struct AgentServiceImpl {
    pool: Arc<Pool<MySql>>,
}

impl AgentServiceImpl {
    pub fn new(pool: Arc<Pool<MySql>>) -> Self {
        AgentServiceImpl { pool }
    }
}

#[async_trait]
impl AgentService for AgentServiceImpl {
    async fn add(&self, user_uuid: &str, agent: AgentAddRequest) -> Result<AgentDbModel, AppError> {
        let agent_model = AgentDbModel {
            uuid: Uuid::new_v4().to_string(),
            user_uuid: user_uuid.to_string(),
            name: agent.name,
            description: agent.description,
            token: Uuid::new_v4().to_string(),
            created_at: Utc::now().timestamp().to_string(),
        };

        let sql = Query::insert()
            .into_table(Agent::Table)
            .columns([
                Agent::Uuid,
                Agent::UserUuid,
                Agent::Name,
                Agent::Description,
                Agent::Token,
                Agent::CreatedAt,
            ])
            .values_panic([
                agent_model.uuid.to_string().into(),
                agent_model.user_uuid.to_string().into(),
                agent_model.name.to_string().into(),
                agent_model.description.to_string().into(),
                agent_model.token.to_string().into(),
                agent_model.created_at.to_string().into(),
            ])
            .to_string(MysqlQueryBuilder)
            .to_owned();

        let mut conn = self.pool.acquire().await.unwrap();

        match conn.execute(&*sql).await {
            Ok(_) => Ok(agent_model),
            Err(e) => {
                error!("Error creating agent: {}", e);
                Err(AppError::InternalServer)
            }
        }
    }

    async fn delete(&self, user_uuid: &str, agent_uuid: &str) -> Result<bool, AppError> {
        let sql = Query::delete()
            .from_table(Agent::Table)
            .and_where(Expr::col(Agent::Uuid).eq(agent_uuid))
            .and_where(Expr::col(Agent::UserUuid).eq(user_uuid))
            .to_string(MysqlQueryBuilder)
            .to_owned();

        let mut conn = self.pool.acquire().await.unwrap();

        match conn.execute(&*sql).await {
            Ok(_) => Ok(true),
            Err(e) => {
                error!("Error deleting agent: {}", e);
                Err(AppError::InternalServer)
            }
        }
    }

    async fn get_all(&self, user_uuid: &str) -> Result<Vec<AgentDbModel>, AppError> {
        let sql = Query::select()
            .columns([
                Agent::Uuid,
                Agent::UserUuid,
                Agent::Name,
                Agent::Description,
                Agent::Token,
                Agent::CreatedAt,
            ])
            .from(Agent::Table)
            .and_where(Expr::col(Agent::UserUuid).eq(user_uuid))
            .to_string(MysqlQueryBuilder);

        let row = sqlx::query(&sql).fetch_all(&*self.pool).await.unwrap();

        if row.is_empty() {
            return Err(AppError::Agent {
                message: "No Agents found".to_string(),
                status: StatusCode::NOT_FOUND,
            });
        }

        let mut agents: Vec<AgentDbModel> = Vec::new();
        for row in row {
            let agent = AgentDbModel::from_row(&row).expect("Error converting row to AgentDbModel");
            agents.push(agent);
        }

        Ok(agents)
    }
}
