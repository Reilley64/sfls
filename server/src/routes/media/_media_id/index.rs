use crate::errors::{Problem, ProblemType};
use crate::middlware::DbConn;
use crate::repositories;
use crate::views::MediaView;
use axum::extract::Path;
use axum::response::IntoResponse;
use axum::Json;
use tracing::error;

pub async fn get(
    DbConn(mut connection): DbConn,
    Path(media_id): Path<String>,
) -> Result<impl IntoResponse, Problem> {
    let instance = Some(format!("/media/{media_id}"));

    let media_id = media_id.parse::<i64>().map_err(|_e| Problem {
        r#type: "https://developer.mozilla.org/en-US/docs/Web/HTTP/Reference/Status/400"
            .to_string(),
        title: "media_id is not a valid id".to_string(),
        status: 400,
        detail: Some(format!("media_id {media_id} is not a valid id")),
        instance: instance.clone(),
    })?;

    Ok(Json(MediaView::from(
        repositories::media::find_by_id(&mut connection, media_id)
            .await
            .map_err(|e| {
                error!("Error while fetching media with id {}: {}", media_id, e);
                Problem::from(ProblemType::InternalServerError(instance.clone()))
            })?
            .ok_or(Problem {
                r#type: "https://developer.mozilla.org/en-US/docs/Web/HTTP/Reference/Status/404"
                    .to_string(),
                title: "Media not found".to_string(),
                status: 400,
                detail: Some(format!("Media with id {media_id} not found")),
                instance: instance.clone(),
            })?,
    )))
}
