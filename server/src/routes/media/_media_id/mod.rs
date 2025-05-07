use crate::state::AppState;
use axum::routing::get;
use axum::Router;

mod images;
mod index;
mod stream;

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/", get(index::get))
        .nest("/images", images::routes())
        .nest("/stream", stream::routes())
}
