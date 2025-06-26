use crate::errors::{Problem, ProblemType};
use crate::middlware::{AuthUser, DbConn};
use crate::models::InsertableHistory;
use crate::repositories;
use axum::extract::Path;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use serde::Deserialize;
use tracing::error;

#[derive(Deserialize)]
pub struct Payload {
    position: i64,
}

pub async fn post(
    DbConn(mut connection): DbConn,
    auth_user: AuthUser,
    Path(media_id): Path<String>,
    Json(body): Json<Payload>,
) -> Result<impl IntoResponse, Problem> {
    let instance = Some("/media/media_id/thumbnail".to_string());

    let media_id = media_id.parse::<i64>().map_err(|_e| Problem {
        r#type: "https://developer.mozilla.org/en-US/docs/Web/HTTP/Reference/Status/400"
            .to_string(),
        title: "media_id is not a valid id".to_string(),
        status: 400,
        detail: Some(format!("media_id {media_id} is not a valid id")),
        instance: instance.clone(),
    })?;

    let media = repositories::media::find_by_id(&mut connection, media_id)
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
        })?;

    if let Err(e) = repositories::history::create(
        &mut connection,
        &InsertableHistory {
            media_id: media.id,
            user_id: auth_user.id,
            position: body.position,
        },
    )
    .await
    {
        error!("Error while creating history: {}", e);
        return Err(ProblemType::InternalServerError(instance).into());
    }

    Ok(StatusCode::OK)
}
