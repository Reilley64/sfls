use async_trait::async_trait;
use domain::errors::AppError;

#[async_trait]
pub trait Job: Send + Sync {
    async fn run(&self) -> Result<(), AppError>;
}
