use crate::common::app_error::AppError;
use crate::services::strategies::file_storage_strategy::FileStorageStrategy;
use axum::async_trait;
use s3::creds::Credentials;
use s3::{Bucket, Region};
use std::env;
use tracing::info;

pub struct S3FileStrategy {}

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
