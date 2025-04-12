use crate::jobs::{scan_folder_job, scan_library_job, JobContext, ScannerFactory};
use crate::r#impl::{OpenAPIImpl, Repositories};
use crate::repositories::library::LibraryRepository;
use crate::repositories::media::MediaRepository;
use diesel::connection::SimpleConnection;
use diesel::{Connection, SqliteConnection};
use effectum::{JobRunner, Queue, Worker};
use snowflake::SnowflakeIdGenerator;
use std::path::PathBuf;
use std::sync::Arc;
use tokio::net::TcpListener;
use tokio::sync::Mutex;
use tracing_subscriber::EnvFilter;

mod r#impl;
mod jobs;
mod libraries;
mod media;
mod models;
mod repositories;
mod schema;
mod nfo;
mod scanners;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    tracing_subscriber::fmt().with_env_filter(EnvFilter::from_default_env()).init();

    let id_generator = Arc::new(Mutex::new(SnowflakeIdGenerator::new(1, 1)));

    let db_url = std::env::var("DATABASE_URL").unwrap_or("sfls.db".to_string());
    let connection = Arc::new(Mutex::new(SqliteConnection::establish(&db_url).unwrap()));
    connection.lock().await.batch_execute("PRAGMA journal_mode = WAL; PRAGMA synchronous = NORMAL;").unwrap();

    let library_repository = LibraryRepository::new(id_generator.clone());
    let media_repository = MediaRepository::new(id_generator.clone());

    let repositories = Arc::new(Repositories {
        library_repository: Box::new(library_repository),
        media_repository: Box::new(media_repository),
    });

    let queue_database_url = std::env::var("QUEUE_DATABASE_URL").unwrap_or("queue.db".to_string());
    let queue = Arc::new(Queue::new(&PathBuf::from(queue_database_url)).await.unwrap());
    let scan_library = JobRunner::builder("scan_library", scan_library_job).build();
    let scan_folder = JobRunner::builder("scan_folder", scan_folder_job).build();
    let queue_context = Arc::new(JobContext {
        connection: connection.clone(),
        repositories: repositories.clone(),
        queue: queue.clone(),
        scanner_factory: ScannerFactory::new(),
    });
    let _worker = Worker::builder(&queue.clone(), queue_context)
        .max_concurrency(10)
        .jobs([scan_library, scan_folder])
        .build()
        .await
        .unwrap();

    let host = std::env::var("SERVER_HOST").unwrap_or("127.0.0.1".to_string());
    let port = std::env::var("SERVER_PORT").unwrap_or("8080".to_string());
    let app = openapi::server::new(OpenAPIImpl {
        connection: connection.clone(),
        repositories: repositories.clone(),
        queue: queue.clone(),
    });
    let listener = TcpListener::bind((host, port.parse().unwrap()))
        .await
        .unwrap();
    axum::serve(listener, app).await.unwrap();
}
