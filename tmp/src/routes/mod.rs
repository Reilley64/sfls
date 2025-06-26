use crate::state::AppState;
use axum::Router;

mod libraries;
mod media;
mod sessions;
mod users;

pub fn routes() -> Router<AppState> {
    Router::new()
        .nest("/media", media::routes())
        .nest("/libraries", libraries::routes())
        .nest("/sessions", sessions::routes())
        .nest("/users", users::routes())
}
