use std::env;

use axum::async_trait;
use s3::creds::Credentials;
use s3::{Bucket, Region};
use tokio::fs;
use tokio::fs::File;
use tokio::io::AsyncWriteExt;
use tracing::{error, info};

use crate::common::app_error::AppError;

pub struct LocalFileStrategy {}

pub struct S3FileStrategy {}

#[async_trait]
pub trait FileStorageStrategy {
    async fn write_file(
        &self,
        user_uuid: &str,
        filename: &str,
        data: &[u8],
    ) -> Result<String, AppError>;
    async fn retrieve_file(&self, filepath: &str) -> Result<Vec<u8>, AppError>;
}

impl S3FileStrategy {
    pub fn get_bucket(&self) -> Bucket {
        let bucket_name = env::var("S3_BUCKET_NAME").expect("S3_BUCKET_NAME must be set!");
        let endpoint = env::var("S3_ENDPOINT").expect("S3_ENDPOINT must be set!");
        let access_key = env::var("S3_ACCESS_KEY").expect("S3_TOKEN must be set!");
        let secret_key = env::var("S3_SECRET_KEY").expect("S3_SECRET_KEY must be set!");

        Bucket::new(
            &bucket_name,
            Region::Custom {
                region: "eu-central-1".to_owned(),
                endpoint: endpoint.to_owned(),
            },
            Credentials::new(Some(&access_key), Some(&secret_key), None, None, None)
                .expect("Failed to retrieve Credentials from S3"),
        )
        .expect("Failed to retrieve Bucket from S3")
        .with_path_style()
    }
}

#[async_trait]
impl FileStorageStrategy for S3FileStrategy {
    async fn write_file(
        &self,
        user_uuid: &str,
        filename: &str,
        data: &[u8],
    ) -> Result<String, AppError> {
        let bucket = self.get_bucket();
        let filepath = format!("{}/{}", user_uuid, filename);

        let response_data = bucket
            .put_object(&filepath, data)
            .await
            .expect("Failed to put object to S3");
        info!("response_data: {:?}", response_data);

        Ok(filepath.to_string())
    }

    async fn retrieve_file(&self, filepath: &str) -> Result<Vec<u8>, AppError> {
        let bucket = self.get_bucket();
        let data = bucket
            .get_object(filepath)
            .await
            .expect("Failed to get object from S3");

        let file_data = data.to_vec();
        Ok(file_data)
    }
}

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
