use sea_query::Iden;
use serde::{Deserialize, Serialize};
use std::fmt;
use std::fmt::Display;

#[derive(Iden)]
#[allow(dead_code)]
pub enum PrintFile {
    Table,
    Uuid,
    UserUuid,
    Name,
    Path,
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
