use crate::state::AppState;
use axum::Router;

mod _file_type;

pub fn routes() -> Router<AppState> {
    Router::new().nest("/{file_type}", _file_type::routes())
}
