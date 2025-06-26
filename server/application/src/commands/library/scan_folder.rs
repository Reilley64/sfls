use crate::commands::command::Command;
use crate::ports::queue::JobQueue;
use async_trait::async_trait;
use domain::errors::AppError;
use domain::repositories::library::LibraryRepository;
use std::path::PathBuf;
use std::sync::Arc;

pub struct ScanFolderCommand {
    pub job_queue: Arc<dyn JobQueue + Send + Sync>,
    pub library_repository: Arc<dyn LibraryRepository + Send + Sync>,
}

impl ScanFolderCommand {
    pub fn new(
        job_queue: Arc<dyn JobQueue + Send + Sync>,
        library_repository: Arc<dyn LibraryRepository + Send + Sync>,
    ) -> Self {
        Self {
            job_queue,
            library_repository,
        }
    }
}

pub struct ScanFolderCommandInput {
    pub library_id: i64,
    pub path: PathBuf,
}

#[async_trait]
impl Command for ScanFolderCommand {
    type Input = ScanFolderCommandInput;
    type Output = ();

    async fn handle(&self, input: Self::Input) -> Result<Self::Output, AppError> {
        tracing::info!("scanning folder: {}", input.path.display());

        let library = self
            .library_repository
            .find_by_id(&input.library_id)
            .await?
            .ok_or(AppError::ValidationError(format!(
                "Library with id {} not found",
                input.library_id
            )))?;

        // self.state
        //     .scanner_factory
        //     .get_scanner(&library.media_type)
        //     .ok_or(anyhow::Error::msg("Unknown media type".to_string()))?
        //     .scan_folder(
        //         self.state.clone(),
        //         library,
        //         Path::new(&self.payload.folder_path),
        //     )
        //     .await?;

        tracing::info!("finished scanning folder: {}", input.path.display());
        Ok(())
    }
}
