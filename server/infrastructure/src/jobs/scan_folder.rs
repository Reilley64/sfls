use std::path::PathBuf;
use application::commands::command::Command;
use application::ports::job::Job;
use application::ports::queue::JobQueue;
use async_trait::async_trait;
use domain::repositories::library::LibraryRepository;
use std::sync::Arc;
use application::commands::library::scan_folder::{ScanFolderCommand, ScanFolderCommandInput};
use domain::errors::AppError;

pub struct ScanFolderJob {
    pub job_queue: Arc<dyn JobQueue + Send + Sync>,
    pub library_repository: Arc<dyn LibraryRepository + Send + Sync>,

    pub library_id: i64,
    pub path: PathBuf,
}

#[async_trait]
impl Job for ScanFolderJob {
    async fn run(&self) -> Result<(), AppError> {
        ScanFolderCommand::new(self.job_queue.clone(), self.library_repository.clone())
            .handle(ScanFolderCommandInput {
                library_id: self.library_id.clone(),
                path: self.path.clone(),
            })
            .await
    }
}
