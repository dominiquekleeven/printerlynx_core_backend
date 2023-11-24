use crate::models::view_model::ViewModel;
use sea_query::Iden;
use serde::{Deserialize, Serialize};

#[derive(Iden)]
pub enum Agent {
    Table,
    Uuid,
    UserUuid,
    Name,
    Description,
    Token,
    CreatedAt,
}

#[derive(sqlx::FromRow, Debug)]
pub struct AgentDbModel {
    pub uuid: String,
    pub user_uuid: String,
    pub name: String,
    pub description: String,
    pub token: String,
    pub created_at: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AgentCreateRequest {
    pub name: String,
    pub description: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AgentViewModel {
    pub uuid: String,
    pub user_uuid: String,
    pub name: String,
    pub description: String,
    pub token: String,
    pub created_at: String,
}

impl ViewModel for AgentDbModel {
    type Model = AgentViewModel;

    fn to_viewmodel(&self) -> Self::Model {
        AgentViewModel {
            uuid: self.uuid.to_string(),
            user_uuid: self.user_uuid.to_string(),
            name: self.name.to_string(),
            description: self.description.to_string(),
            token: self.token.to_string(),
            created_at: self.created_at.to_string(),
        }
    }
}
