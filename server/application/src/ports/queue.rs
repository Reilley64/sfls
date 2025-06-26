use crate::ports::job::Job;
use async_trait::async_trait;

#[async_trait]
pub trait JobQueue {
    async fn enqueue(&self, job: Box<dyn Job + Send + Sync>) -> Result<(), anyhow::Error>;
}
