use crate::models::view_model::ViewModel;
use sea_query::Iden;
use serde::{Deserialize, Serialize};

#[derive(Iden)]
pub enum Account {
    Table,
    Uuid,
    Username,
    Email,
    Password,
    CreatedAt,
    UpdatedAt,
}

#[derive(sqlx::FromRow, Debug)]
pub struct AccountDbModel {
    pub uuid: String,
    pub username: String,
    pub email: String,
    pub password: String,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AccountViewModel {
    pub uuid: String,
    pub username: String,
    pub email: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AccountRegisterModel {
    pub username: String,
    pub email: String,
    pub password: String,
    pub password_confirmation: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AccountLoginModel {
    pub username: String,
    pub password: String,
}

impl ViewModel for AccountDbModel {
    type Model = AccountViewModel;

    fn to_viewmodel(&self) -> Self::Model {
        AccountViewModel {
            uuid: self.uuid.to_string(),
            username: self.username.to_string(),
            email: self.email.to_string(),
        }
    }
}
