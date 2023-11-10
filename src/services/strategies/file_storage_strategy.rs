use axum::async_trait;

use crate::common::app_error::AppError;

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
