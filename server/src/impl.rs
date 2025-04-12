use crate::repositories::library::LibraryRepositoryTrait;
use crate::repositories::media::MediaRepositoryTrait;
use diesel::SqliteConnection;
use effectum::Queue;
use openapi::apis::ErrorHandler;
use std::fmt;
use std::fmt::{Debug, Formatter};
use std::sync::Arc;
use tokio::sync::Mutex;

pub struct Repositories {
    pub library_repository: Box<dyn LibraryRepositoryTrait + Send + Sync>,
    pub media_repository: Box<dyn MediaRepositoryTrait + Send + Sync>,
}

impl Debug for Repositories {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_struct("Repositories")
            .field("library_repository", &"<dyn LibraryRepositoryTrait>")
            .field("media_repository", &"<dyn MediaRepositoryTrait>")
            .finish()
    }
}


#[derive(Clone)]
pub struct OpenAPIImpl {
    pub connection: Arc<Mutex<SqliteConnection>>,
    pub repositories: Arc<Repositories>,
    pub queue: Arc<Queue>,
}

impl AsRef<OpenAPIImpl> for OpenAPIImpl {
    fn as_ref(&self) -> &OpenAPIImpl {
        self
    }
}

impl ErrorHandler<()> for OpenAPIImpl {}
