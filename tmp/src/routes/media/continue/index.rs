use crate::errors::{Problem, ProblemType};
use crate::middlware::{AuthUser, DbConn};
use crate::repositories;
use crate::views::MediaView;
use axum::response::IntoResponse;
use axum::Json;
use tracing::error;

pub async fn get(
    DbConn(mut connection): DbConn,
    auth_user: AuthUser,
) -> Result<impl IntoResponse, Problem> {
    let instance = Some("/media/continue".to_string());

    Ok(Json(
        repositories::media::find_continue_watching(&mut connection, auth_user.id)
            .await
            .map_err(|e| {
                error!("Error while fetching continue watching: {}", e);
                Problem::from(ProblemType::InternalServerError(instance))
            })?
            .into_iter()
            .map(MediaView::from)
            .collect::<Vec<_>>(),
    ))
}
