use std::fmt;
use std::fmt::Display;

use crate::models::view_model::ViewModel;
use sea_query::Iden;
use serde::{Deserialize, Serialize};

#[derive(Iden)]
pub enum PrintFile {
    Table,
    Uuid,
    UserUuid,
    Name,
    Path,
    Size,
    Checksum,
    FileType,
    FileStorageType,
    CreatedAt,
}

#[derive(sqlx::FromRow, Debug)]
pub struct PrintFileDbModel {
    pub uuid: String,
    pub user_uuid: String,
    pub name: String,
    pub path: String,
    pub size: i32,
    pub checksum: String,
    pub file_type: String,
    pub file_storage_type: String,
    pub created_at: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PrintFileViewModel {
    pub uuid: String,
    pub user_uuid: String,
    pub name: String,
    pub size: i32,
    pub checksum: String,
    pub file_type: String,
    pub file_storage_type: String,
    pub created_at: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum FileType {
    Gcode,
    Stl,
    Obj,
    Amf,
    Unknown,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum FileStorageType {
    Local,
    S3,
}

impl Display for FileStorageType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Display for FileType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl ViewModel for PrintFileDbModel {
    type Model = PrintFileViewModel;

    fn to_viewmodel(&self) -> Self::Model {
        PrintFileViewModel {
            uuid: self.uuid.to_string(),
            user_uuid: self.user_uuid.to_string(),
            name: self.name.to_string(),
            size: self.size.to_owned(),
            checksum: self.checksum.to_string(),
            file_type: self.file_type.to_string(),
            file_storage_type: self.file_storage_type.to_string(),
            created_at: self.created_at.to_string(),
        }
    }
}
