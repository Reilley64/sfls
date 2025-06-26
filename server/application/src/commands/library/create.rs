use crate::commands::command::Command;
use crate::ports::job_factory::JobFactory;
use crate::ports::queue::JobQueue;
use async_trait::async_trait;
use domain::entities::library::Library;
use domain::entities::media_type::MediaType;
use domain::entities::user::User;
use domain::repositories::library::LibraryRepository;
use std::path::PathBuf;
use std::sync::Arc;
use domain::errors::AppError;

pub struct CreateLibraryCommand {
    pub job_queue: Arc<dyn JobQueue + Send + Sync>,
    pub job_factory: Arc<dyn JobFactory + Send + Sync>,
    pub library_repository: Arc<dyn LibraryRepository + Send + Sync>,
}

impl CreateLibraryCommand {
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

pub struct CreateLibraryCommandInput {
    pub user: User,
    pub name: String,
    pub path: PathBuf,
    pub media_type: MediaType,
}

#[async_trait]
impl Command for CreateLibraryCommand {
    type Input = CreateLibraryCommandInput;
    type Output = Library;

    async fn handle(&self, input: Self::Input) -> Result<Self::Output, AppError> {
        if !input.path.exists() {
            return Err(AppError::ValidationError("Library path does not exist".to_string()));
        }

        if !input.path.is_dir() {
            return Err(AppError::ValidationError("Library path is not a directory".to_string()));
        }

        let library = self
            .library_repository
            .create(&Library {
                created_by: input.user.name.clone(),
                updated_by: input.user.name.clone(),
                name: input.name.clone(),
                path: input
                    .path
                    .clone()
                    .to_str()
                    .ok_or(AppError::ValidationError("Invalid path".to_string()))?
                    .to_string(),
                media_type: input.media_type.clone(),
                ..Default::default()
            })
            .await?;

        self.job_queue
            .enqueue(self.job_factory.create_scan_library_job(library.id))
            .await?;

        Ok(library)
    }
}
