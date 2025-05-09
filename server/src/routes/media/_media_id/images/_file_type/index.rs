use crate::errors::{Problem, ProblemType};
use crate::middlware::DbConn;
use crate::models::FileType;
use crate::repositories;
use axum::body::Body;
use axum::extract::Path;
use axum::http::{header, HeaderMap, StatusCode};
use axum::response::IntoResponse;
use std::path::PathBuf;
use tokio::fs::File;
use tokio_util::io::ReaderStream;
use tracing::error;

pub async fn get(
    DbConn(mut connection): DbConn,
    Path((media_id, file_type)): Path<(String, String)>,
) -> Result<impl IntoResponse, Problem> {
    let instance = Some(format!("/media/{media_id}/fanart"));

    let media_id = media_id.parse::<i64>().map_err(|_e| Problem {
        r#type: "https://developer.mozilla.org/en-US/docs/Web/HTTP/Reference/Status/400"
            .to_string(),
        title: "media_id is not a valid id".to_string(),
        status: 400,
        detail: Some(format!("media_id {media_id} is not a valid id")),
        instance: instance.clone(),
    })?;

    let file_type = file_type.parse::<FileType>().map_err(|_e| Problem {
        r#type: "https://developer.mozilla.org/en-US/docs/Web/HTTP/Reference/Status/400"
            .to_string(),
        title: "file_type is not a valid file_type".to_string(),
        status: 400,
        detail: Some(format!("file_type {file_type} is not a valid file_type")),
        instance: instance.clone(),
    })?;

    let mut media = repositories::media::find_by_id(&mut connection, media_id)
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

    let image = media
        .files
        .iter()
        .find(|f| f.type_ == file_type)
        .ok_or(Problem {
            r#type: "https://developer.mozilla.org/en-US/docs/Web/HTTP/Reference/Status/500"
                .to_string(),
            title: format!("{:?} not found", file_type),
            status: 404,
            detail: Some(format!("Fanart for Media with id {media_id} not found")),
            instance: instance.clone(),
        })?;

    let mut components = vec![Some(image.path.clone()), media.path];
    while let Some(parent_id) = media.parent_id {
        media = repositories::media::find_by_id(&mut connection, parent_id)
            .await
            .map_err(|e| {
                error!("Error while fetching media with id {}: {}", parent_id, e);
                Problem::from(ProblemType::InternalServerError(instance.clone()))
            })?
            .ok_or(Problem {
                r#type: "https://developer.mozilla.org/en-US/docs/Web/HTTP/Reference/Status/404"
                    .to_string(),
                title: "Media not found".to_string(),
                status: 404,
                detail: Some(format!("Media with id {parent_id} not found")),
                instance: instance.clone(),
            })?;

        components.push(media.path);
    }

    let path = components
        .into_iter()
        .rev()
        .flatten()
        .fold(PathBuf::new(), |acc, component| acc.join(component));

    let file = File::open(path.clone()).await.map_err(|e| {
        error!(
            "Error while reading fanart file {}: {}",
            path.to_str().unwrap(),
            e
        );
        Problem::from(ProblemType::InternalServerError(instance.clone()))
    })?;

    let stream = ReaderStream::new(file);

    let mut response_headers = HeaderMap::new();
    response_headers.insert(header::CONTENT_TYPE, "images/webp".parse().unwrap());

    Ok((StatusCode::OK, response_headers, Body::from_stream(stream)))
}
