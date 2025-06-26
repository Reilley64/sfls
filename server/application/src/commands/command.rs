use async_trait::async_trait;
use domain::errors::AppError;

#[async_trait]
pub trait Command {
    type Input;
    type Output;
    
    async fn handle(&self, input: Self::Input) -> Result<Self::Output, AppError>;
}
