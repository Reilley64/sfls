use application::commands::command::Command;
use application::commands::library::scan::{ScanLibraryCommand, ScanLibraryCommandInput};
use application::ports::job::Job;
use application::ports::queue::JobQueue;
use async_trait::async_trait;
use domain::repositories::library::LibraryRepository;
use std::sync::Arc;
use application::ports::job_factory::JobFactory;
use domain::errors::AppError;

pub struct ScanLibraryJob {
    pub job_queue: Arc<dyn JobQueue + Send + Sync>,
    pub job_factory: Arc<dyn JobFactory + Send + Sync>,
    pub library_repository: Arc<dyn LibraryRepository + Send + Sync>,

    pub library_id: i64,
}

#[async_trait]
impl Job for ScanLibraryJob {
    async fn run(&self) -> Result<(), AppError> {
        ScanLibraryCommand::new(self.job_queue.clone(), self.job_factory.clone(), self.library_repository.clone())
            .handle(ScanLibraryCommandInput {
                library_id: self.library_id.clone(),
            })
            .await
    }
}
