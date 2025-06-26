use crate::commands::command::Command;
use crate::ports::queue::JobQueue;
use async_trait::async_trait;
use domain::errors::AppError;
use domain::repositories::library::LibraryRepository;
use std::path::Path;
use std::sync::Arc;
use tokio::fs;
use crate::ports::job_factory::JobFactory;

pub struct ScanLibraryCommand {
    pub job_queue: Arc<dyn JobQueue + Send + Sync>,
    pub job_factory: Arc<dyn JobFactory + Send + Sync>,
    pub library_repository: Arc<dyn LibraryRepository + Send + Sync>,
}

impl ScanLibraryCommand {
    pub fn new(
        job_queue: Arc<dyn JobQueue + Send + Sync>,
        job_factory: Arc<dyn JobFactory + Send + Sync>,
        library_repository: Arc<dyn LibraryRepository + Send + Sync>,
    ) -> Self {
        Self {
            job_queue,
            job_factory,
            library_repository,
        }
    }
}

pub struct ScanLibraryCommandInput {
    pub library_id: i64,
}

#[async_trait]
impl Command for ScanLibraryCommand {
    type Input = ScanLibraryCommandInput;
    type Output = ();

    async fn handle(&self, input: Self::Input) -> Result<Self::Output, AppError> {
        tracing::info!("scanning library: {}", input.library_id);

        let library = self
            .library_repository
            .find_by_id(&input.library_id)
            .await?
            .ok_or(AppError::ValidationError(format!(
                "Library with id {} not found",
                input.library_id
            )))?;

        let path = Path::new(&library.path);

        let mut dir = fs::read_dir(path).await?;

        while let Some(entry) = dir.next_entry().await? {
            if !entry.path().is_dir() {
                continue;
            }

            self.job_queue
                .enqueue(self.job_factory.create_scan_folder_job(library.id, entry.path()))
                .await?;
        }

        tracing::info!("finished scanning library: {}", input.library_id);
        Ok(())
    }
}
