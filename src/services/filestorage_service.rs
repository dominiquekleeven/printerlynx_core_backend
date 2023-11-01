use std::env;

use tracing::error;

use crate::common::app_error::AppError;
use crate::services::strategies::file_storage_strategy::{
    FileStorageStrategy, LocalFileStrategy, S3FileStrategy,
};

pub async fn store_file(user_uuid: &str, filename: &str, data: &[u8]) -> Result<String, AppError> {
    let file_storage_type = env::var("FILESTORAGE_TYPE").expect("FILESTORAGE_TYPE must be set!");

    match file_storage_type.as_str() {
        "local" => {
            LocalFileStrategy::write_file(&LocalFileStrategy {}, user_uuid, filename, data).await
        }
        "s3" => S3FileStrategy::write_file(&S3FileStrategy {}, user_uuid, filename, data).await,
        _ => {
            error!("unknown file storage type: {}", file_storage_type);
            Err(AppError::InternalServer {
                message: "Something went wrong during the file upload".to_string(),
            })
        }
    }
}
