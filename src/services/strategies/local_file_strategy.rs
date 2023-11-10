use std::env;

use axum::async_trait;
use tokio::fs;
use tokio::fs::File;
use tokio::io::AsyncWriteExt;
use tracing::{error, info};

use crate::common::app_error::AppError;
use crate::services::strategies::file_storage_strategy::FileStorageStrategy;

pub struct LocalFileStrategy {}

#[async_trait]
impl FileStorageStrategy for LocalFileStrategy {
    async fn write_file(
        &self,
        user_uuid: &str,
        filename: &str,
        data: &[u8],
    ) -> Result<String, AppError> {
        let base_directory = env::var("FILESTORAGE_PATH").expect("FILESTORAGE_PATH must be set!");

        let directory = format!("{}/{}", base_directory, user_uuid);
        fs::create_dir_all(&directory).await.unwrap(); // We don't care if the directory already exists

        let filepath = format!("{}/{}", directory, filename);
        let mut file = File::create(&filepath).await.unwrap();
        match file.write_all(data).await {
            Ok(_) => {
                info!("file {} written successfully", filename);
                Ok(filepath.to_string())
            }
            Err(e) => {
                error!("error writing file {}: {}", filename, e);
                Err(AppError::InternalServer {
                    message: "Something went wrong during the file upload".to_string(),
                })
            }
        }
    }

    async fn retrieve_file(&self, filepath: &str) -> Result<Vec<u8>, AppError> {
        let base_directory = env::var("FILESTORAGE_PATH").expect("FILESTORAGE_PATH must be set!");
        let filepath = format!("{}/{}", base_directory, filepath);
        let data = fs::read(filepath).await.unwrap();
        Ok(data)
    }
}
