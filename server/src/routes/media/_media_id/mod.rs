use crate::state::AppState;
use axum::routing::get;
use axum::Router;

mod fanart;
mod index;
mod poster;
mod stream;
mod thumbnail;

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/", get(index::get))
        .nest("/fanart", fanart::routes())
        .nest("/poster", poster::routes())
        .nest("/stream", stream::routes())
        .nest("/thumbnail", thumbnail::routes())
}
