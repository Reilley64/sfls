use deadpool::managed::Pool;
use diesel_async::pooled_connection::AsyncDieselConnectionManager;
use diesel_async::AsyncPgConnection;
use std::sync::Arc;
use application::ports::job_factory::JobFactory;
use application::ports::queue::JobQueue;
use domain::repositories::library::LibraryRepository;

pub type DbPool = Pool<AsyncDieselConnectionManager<AsyncPgConnection>>;

#[derive(Clone)]
pub struct AppState {
    pub secret: Vec<u8>,
    pub job_queue: Arc<dyn JobQueue + Send + Sync>,
    pub job_factory: Arc<dyn JobFactory + Send + Sync>,
    pub repositories: Repositories,
}

#[derive(Clone)]
pub struct Repositories {
    pub library_repository: Arc<dyn LibraryRepository + Send + Sync>,
}
