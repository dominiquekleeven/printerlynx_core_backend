use std::env;

use tracing::error;

use crate::common::app_error::AppError;
use crate::infra::strategies::file_storage_strategy::FileStorageStrategy;
use crate::infra::strategies::local_file_strategy::LocalFileStrategy;
use crate::infra::strategies::s3_file_strategy::S3FileStrategy;

pub async fn store_file(user_uuid: &str, filename: &str, data: &[u8]) -> Result<String, AppError> {
    let file_storage_type = env::var("FILESTORAGE_TYPE").expect("FILESTORAGE_TYPE must be set!");

    match file_storage_type.as_str() {
        "local" => {
            LocalFileStrategy::write_file(&LocalFileStrategy {}, user_uuid, filename, data).await
        }
        "s3" => S3FileStrategy::write_file(&S3FileStrategy {}, user_uuid, filename, data).await,
        _ => {
            error!("unknown file storage type: {}", file_storage_type);
            Err(AppError::InternalServer)
        }
    }
}

pub async fn retrieve_file(filepath: &str) -> Result<Vec<u8>, AppError> {
    let file_storage_type = env::var("FILESTORAGE_TYPE").expect("FILESTORAGE_TYPE must be set!");

    match file_storage_type.as_str() {
        "local" => LocalFileStrategy::retrieve_file(&LocalFileStrategy {}, filepath).await,
        "s3" => S3FileStrategy::retrieve_file(&S3FileStrategy {}, filepath).await,
        _ => {
            error!("unknown file storage type: {}", file_storage_type);
            Err(AppError::InternalServer)
        }
    }
}