use crate::state::AppState;
use axum::routing::post;
use axum::Router;

mod index;

pub fn routes() -> Router<AppState> {
    Router::new().route("/", post(index::post))
}
