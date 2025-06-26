use axum::Router;
use axum::routing::post;
use tokio::net::TcpListener;
use tower_http::trace::TraceLayer;
use tracing_subscriber::EnvFilter;
use infrastructure::di;

pub mod controllers {
    pub mod library;
}

pub mod errors;

pub mod models {
    pub mod library;
}

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();
    
    tracing::info!("starting services");
    let state = di::configure_services()?;

    tracing::info!("starting server");
    let app = Router::new()
        .route("/library", post(controllers::library::create_library_handler))
        .with_state(state)
        .layer(TraceLayer::new_for_http());
    let host = std::env::var("SERVER_HOST").unwrap_or("0.0.0.0".to_string());
    let port = std::env::var("SERVER_PORT").unwrap_or("8080".to_string());
    tracing::info!("listening on {}:{}", host, port);
    let listener = TcpListener::bind((host, port.parse().unwrap()))
        .await?;
    axum::serve(listener, app).await?;
    
    Ok(())
}
