use std::sync::Arc;
use diesel_async::AsyncPgConnection;
use diesel_async::pooled_connection::AsyncDieselConnectionManager;
use diesel_async::pooled_connection::deadpool::Pool;
use rand::Rng;
use crate::jobs::factory::JobFactoryImpl;
use crate::persistence::repositories::library::LibraryRepositoryImpl;
use crate::queues::tokio::TokioQueue;
use crate::state::{AppState, Repositories};

pub fn configure_services() -> Result<AppState, anyhow::Error> {
    tracing::info!("generating new authorization secret");
    let mut secret = vec![0u8; 32];
    rand::rng().fill(&mut secret[..]);
    
    tracing::info!("connecting to database");
    let db_url = std::env::var("DATABASE_URL").map_err(|_e| anyhow::anyhow!("DATABASE_URL not set"))?;
    let manager = AsyncDieselConnectionManager::<AsyncPgConnection>::new(db_url);
    let pool = Pool::builder(manager).build()?;
    
    tracing::info!("creating repositories");
    let library_repository = Arc::new(LibraryRepositoryImpl::new(pool.clone()));
    let repositories = Repositories {
      library_repository: library_repository.clone(),  
    };
    
    tracing::info!("starting job queue");
    let job_queue = Arc::new(TokioQueue::new());
    
    tracing::info!("creating job factory");
    let job_factory = Arc::new(JobFactoryImpl::new(job_queue.clone(), library_repository.clone()));
    
    Ok(AppState {
        secret,
        job_queue,
        job_factory,
        repositories
    })
}