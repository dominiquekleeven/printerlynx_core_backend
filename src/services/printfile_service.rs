use std::env;
use std::sync::Arc;

use axum::async_trait;
use axum::extract::Multipart;
use axum::http::StatusCode;
use chrono::Utc;
use sea_query::{Expr, MysqlQueryBuilder, Query};
use sha2::Digest;
use sqlx::{Executor, FromRow, MySql, Pool};
use tracing::{error, info};
use uuid::Uuid;

use crate::common::app_error::AppError;
use crate::infra::filestorage::{retrieve_file, store_file};
use crate::models::printfile_model::{FileType, PrintFile, PrintFileDbModel};

#[async_trait]
pub trait PrintFileService {
    async fn upload(
        &self,
        user_uuid: &str,
        multipart_file: Multipart,
    ) -> Result<PrintFileDbModel, AppError>;
    async fn delete(&self, user_uuid: &str, file_uuid: &str) -> Result<bool, AppError>;
    async fn get_all(&self, user_uuid: &str) -> Result<Vec<PrintFileDbModel>, AppError>;
    async fn get_by_uuid(
        &self,
        user_uuid: &str,
        file_uuid: &str,
    ) -> Result<PrintFileDbModel, AppError>;
    async fn download(&self, user_uuid: &str, file_uuid: &str) -> Result<Vec<u8>, AppError>;
}

pub struct PrintFileServiceImpl {
    pool: Arc<Pool<MySql>>,
}

impl PrintFileServiceImpl {
    pub fn new(pool: Arc<Pool<MySql>>) -> Self {
        PrintFileServiceImpl { pool }
    }
}

#[async_trait]
impl PrintFileService for PrintFileServiceImpl {
    async fn upload(
        &self,
        user_uuid: &str,
        mut multipart_file: Multipart,
    ) -> Result<PrintFileDbModel, AppError> {
        let mut printfile = None;
        while let Some(field) = multipart_file.next_field().await.unwrap() {
            let filename = &field.file_name().unwrap().to_string();
            let data = &field.bytes().await.unwrap();
            let filesize = data.len() as i32;
            let filepath = store_file(user_uuid, filename, data).await?;
            let sha256 = format!("{:x}", sha2::Sha256::digest(data));

            info!(
                "success, filepath: {}, sha256 checksum: {}, size: {}",
                filepath, sha256, filesize
            );
            printfile = Some(
                insert_printfile(
                    self.pool.clone(),
                    user_uuid,
                    filename,
                    &filepath,
                    filesize,
                    &sha256,
                )
                .await?,
            );
        }

        match printfile {
            Some(printfile) => Ok(printfile),
            None => Err(AppError::InternalServer),
        }
    }

    async fn delete(&self, user_uuid: &str, file_uuid: &str) -> Result<bool, AppError> {
        let sql = Query::delete()
            .from_table(PrintFile::Table)
            .and_where(Expr::col(PrintFile::UserUuid).eq(user_uuid))
            .and_where(Expr::col(PrintFile::Uuid).eq(file_uuid))
            .to_string(MysqlQueryBuilder);

        let mut conn = self.pool.acquire().await.unwrap();
        match conn.execute(&*sql).await {
            Ok(_) => Ok(true),
            Err(e) => {
                error!("Error deleting printfile: {}", e);
                Err(AppError::InternalServer)
            }
        }
    }

    async fn get_all(&self, user_uuid: &str) -> Result<Vec<PrintFileDbModel>, AppError> {
        let sql = Query::select()
            .columns([
                PrintFile::Uuid,
                PrintFile::UserUuid,
                PrintFile::Name,
                PrintFile::Path,
                PrintFile::Size,
                PrintFile::Checksum,
                PrintFile::FileType,
                PrintFile::FileStorageType,
                PrintFile::CreatedAt,
            ])
            .from(PrintFile::Table)
            .and_where(Expr::col(PrintFile::UserUuid).eq(user_uuid))
            .to_string(MysqlQueryBuilder);

        let row = sqlx::query(&sql).fetch_all(&*self.pool).await.unwrap();

        if row.is_empty() {
            return Err(AppError::PrintFile {
                message: "No files found".to_string(),
                status: StatusCode::NOT_FOUND,
            });
        }

        let mut printfiles: Vec<PrintFileDbModel> = Vec::new();
        for row in row {
            let printfile =
                PrintFileDbModel::from_row(&row).expect("Error converting row to PrintFileDbModel");
            printfiles.push(printfile);
        }

        Ok(printfiles)
    }

    async fn get_by_uuid(
        &self,
        user_uuid: &str,
        file_uuid: &str,
    ) -> Result<PrintFileDbModel, AppError> {
        let sql = Query::select()
            .columns([
                PrintFile::Uuid,
                PrintFile::UserUuid,
                PrintFile::Name,
                PrintFile::Path,
                PrintFile::Size,
                PrintFile::Checksum,
                PrintFile::FileType,
                PrintFile::FileStorageType,
                PrintFile::CreatedAt,
            ])
            .from(PrintFile::Table)
            .and_where(Expr::col(PrintFile::UserUuid).eq(user_uuid))
            .and_where(Expr::col(PrintFile::Uuid).eq(file_uuid))
            .to_string(MysqlQueryBuilder);

        let row = sqlx::query(&sql).fetch_optional(&*self.pool).await.unwrap();

        if row.is_none() {
            return Err(AppError::PrintFile {
                message: "No file found".to_string(),
                status: StatusCode::NOT_FOUND,
            });
        }

        let row = row.expect("Error unwrapping row");
        let printfile =
            PrintFileDbModel::from_row(&row).expect("Error converting row to PrintFileDbModel");

        Ok(printfile)
    }

    async fn download(&self, user_uuid: &str, file_uuid: &str) -> Result<Vec<u8>, AppError> {
        let printfile = self.get_by_uuid(user_uuid, file_uuid).await?;
        Ok(retrieve_file(&printfile.path).await?)
    }
}

//TODO: ask for overwrite
async fn insert_printfile(
    pool: Arc<Pool<MySql>>,
    user_uuid: &str,
    filename: &str,
    filepath: &str,
    filesize: i32,
    sha256: &str,
) -> Result<PrintFileDbModel, AppError> {
    let file_storage_type = env::var("FILESTORAGE_TYPE").expect("FILESTORAGE_TYPE must be set!");

    let printfile_model = PrintFileDbModel {
        uuid: Uuid::new_v4().to_string(),
        user_uuid: user_uuid.to_string(),
        name: filename.to_string(),
        path: filepath.to_string(),
        size: filesize.to_owned(),
        checksum: sha256.to_string(),
        file_type: FileType::Gcode.to_string(),
        file_storage_type: file_storage_type.to_string(),
        created_at: Utc::now().timestamp().to_string(),
    };

    let sql = Query::insert()
        .into_table(PrintFile::Table)
        .columns([
            PrintFile::Uuid,
            PrintFile::UserUuid,
            PrintFile::Name,
            PrintFile::Path,
            PrintFile::Size,
            PrintFile::Checksum,
            PrintFile::FileType,
            PrintFile::FileStorageType,
            PrintFile::CreatedAt,
        ])
        .values_panic([
            printfile_model.uuid.to_string().into(),
            printfile_model.user_uuid.to_string().into(),
            printfile_model.name.to_string().into(),
            printfile_model.path.to_string().into(),
            printfile_model.size.into(),
            printfile_model.checksum.to_string().into(),
            printfile_model.file_type.to_string().into(),
            printfile_model.file_storage_type.to_string().into(),
            printfile_model.created_at.to_string().into(),
        ])
        .to_string(MysqlQueryBuilder)
        .to_owned();

    let mut conn = pool.acquire().await.unwrap();

    match conn.execute(&*sql).await {
        Ok(_) => Ok(printfile_model),
        Err(e) => {
            error!("Error inserting printfile: {}", e);
            Err(AppError::InternalServer)
        }
    }
}
