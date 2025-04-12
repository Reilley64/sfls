use crate::factories::artwork_fetcher::ArtworkFetcherFactory;
use crate::factories::library_scanner::ScannerFactory;
use crate::jobs::Job;
use crate::state::AppState;
use axum::Router;
use diesel_async::pooled_connection::deadpool::Pool;
use diesel_async::pooled_connection::AsyncDieselConnectionManager;
use diesel_async::AsyncPgConnection;
use openssl::pkey::PKey;
use std::path::Path;
use std::sync::Arc;
use tokio::net::TcpListener;
use tokio::sync::{mpsc, Semaphore};
use tower_http::trace::TraceLayer;
use tracing::info;
use tracing_subscriber::EnvFilter;

mod clients;
mod errors;
mod factories;
mod jobs;
mod middlware;
mod models;
mod nfo;
mod repositories;
mod routes;
mod schema;
mod state;
mod views;

const PRIVATE_KEY_PATH: &str = "private.pem";
const PUBLIC_KEY_PATH: &str = "public.pem";

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    let (public_key, private_key) =
        if Path::new(PRIVATE_KEY_PATH).exists() && Path::new(PUBLIC_KEY_PATH).exists() {
            info!("Loading existing authorization keys");
            (
                tokio::fs::read(PUBLIC_KEY_PATH).await.unwrap(),
                tokio::fs::read(PRIVATE_KEY_PATH).await.unwrap(),
            )
        } else {
            info!("Generating new authorization keys");
            let key = PKey::generate_ed25519().unwrap();
            let private_key = key.private_key_to_pem_pkcs8().unwrap();
            tokio::fs::write(PRIVATE_KEY_PATH, private_key.clone())
                .await
                .unwrap();
            let public_key = key.public_key_to_pem().unwrap();
            tokio::fs::write(PUBLIC_KEY_PATH, public_key.clone())
                .await
                .unwrap();
            (private_key, public_key)
        };

    info!("Connecting to database");
    let db_url = std::env::var("DATABASE_URL").unwrap();
    let manager = AsyncDieselConnectionManager::<AsyncPgConnection>::new(db_url);
    let pool = Pool::builder(manager).build().unwrap();

    info!("Starting job queue");
    let (tx, mut rx) = mpsc::unbounded_channel::<Box<dyn Job + Send + Sync>>();
    tokio::spawn(async move {
        let semaphore = Arc::new(Semaphore::new(num_cpus::get()));

        while let Some(job) = rx.recv().await {
            let semaphore = semaphore.clone();

            tokio::spawn(async move {
                let _permit = match semaphore.acquire().await {
                    Ok(p) => p,
                    Err(_e) => {
                        tracing::error!("Failed to acquire semaphore");
                        return;
                    }
                };

                if let Err(e) = job.run().await {
                    tracing::error!("Failed to run job: {}", e);
                }
            });
        }
    });

    let state = AppState {
        public_key,
        private_key,
        pool,
        queue: tx,
        artwork_fetcher_factory: Arc::new(ArtworkFetcherFactory::default()),
        scanner_factory: Arc::new(ScannerFactory::default()),
    };

    info!("Starting server");
    let app = Router::new()
        .merge(routes::routes())
        .with_state(state)
        .layer(TraceLayer::new_for_http());

    let host = std::env::var("SERVER_HOST").unwrap_or("0.0.0.0".to_string());
    let port = std::env::var("SERVER_PORT").unwrap_or("8080".to_string());
    info!("Listening on {}:{}", host, port);
    let listener = TcpListener::bind((host, port.parse().unwrap()))
        .await
        .unwrap();
    axum::serve(listener, app).await.unwrap();
}
