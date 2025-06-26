use crate::state::AppState;
use axum::routing::get;
use axum::Router;

mod _media_id;
mod r#continue;
mod index;

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/", get(index::get))
        .nest("/{media_id}", _media_id::routes())
        .nest("/continue", r#continue::routes())
}
