use sea_query::Iden;
use serde::{Deserialize, Serialize};
use std::fmt;
use std::fmt::Display;

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

pub fn printfile_to_viewmodel(printfile: PrintFileDbModel) -> PrintFileViewModel {
    PrintFileViewModel {
        uuid: printfile.uuid,
        user_uuid: printfile.user_uuid,
        name: printfile.name,
        size: printfile.size,
        checksum: printfile.checksum,
        file_type: printfile.file_type,
        file_storage_type: printfile.file_storage_type,
        created_at: printfile.created_at,
    }
}
