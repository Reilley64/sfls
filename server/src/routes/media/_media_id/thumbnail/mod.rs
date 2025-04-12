mod index;

use crate::state::AppState;
use axum::routing::get;
use axum::Router;

pub fn routes() -> Router<AppState> {
    Router::new().route("/", get(index::get))
}
