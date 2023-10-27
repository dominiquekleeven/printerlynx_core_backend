use crate::models::view_model::ViewModel;
use serde::{Deserialize, Serialize};

#[allow(dead_code)]
pub enum Agent {
    Table,
    Uuid,
    UserUuid,
    Name,
    Token,
    Ip,
    Port,
    CreatedAt,
}

#[derive(sqlx::FromRow, Debug)]
pub struct AgentDbModel {
    pub uuid: String,
    pub user_uuid: String,
    pub name: String,
    pub token: String,
    pub ip: String,
    pub port: i32,
    pub created_at: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AgentViewModel {
    pub uuid: String,
    pub user_uuid: String,
    pub name: String,
    pub token: String,
    pub ip: String,
    pub port: i32,
    pub created_at: String,
}

impl ViewModel for AgentDbModel {
    type Model = AgentViewModel;

    fn to_viewmodel(&self) -> Self::Model {
        AgentViewModel {
            uuid: self.uuid.to_string(),
            user_uuid: self.user_uuid.to_string(),
            name: self.name.to_string(),
            token: self.token.to_string(),
            ip: self.ip.to_string(),
            port: self.port.to_owned(),
            created_at: self.created_at.to_string(),
        }
    }
}
