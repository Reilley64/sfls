use crate::state::AppState;
use axum::routing::get;
use axum::Router;

mod heartbeat;
mod index;

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/", get(index::get))
        .nest("/heartbeat", heartbeat::routes())
}
