use crate::factories::artwork_fetcher::ArtworkFetcherFactory;
use crate::factories::library_scanner::ScannerFactory;
use crate::jobs::Job;
use deadpool::managed::Pool;
use diesel_async::pooled_connection::AsyncDieselConnectionManager;
use diesel_async::AsyncPgConnection;
use std::sync::Arc;
use tokio::sync::mpsc::UnboundedSender;

pub type DbPool = Pool<AsyncDieselConnectionManager<AsyncPgConnection>>;

#[derive(Clone)]
pub struct AppState {
    pub secret: Vec<u8>,
    pub pool: DbPool,
    pub queue: UnboundedSender<Box<dyn Job + Send + Sync>>,
    pub artwork_fetcher_factory: Arc<ArtworkFetcherFactory>,
    pub scanner_factory: Arc<ScannerFactory>,
}
