use crate::errors::{Problem, ProblemType};
use crate::middlware::{AuthUser, DbConn};
use crate::repositories;
use axum::body::Body;
use axum::extract::Path;
use axum::http::{header, HeaderMap, StatusCode};
use axum::response::IntoResponse;
use std::io::SeekFrom;
use std::path::PathBuf;
use tokio::fs::File;
use tokio::io::{AsyncReadExt, AsyncSeekExt};
use tokio_util::io::ReaderStream;
use tracing::error;

pub async fn get(
    DbConn(mut connection): DbConn,
    _: AuthUser,
    Path(media_id): Path<String>,
    headers: HeaderMap,
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

    if media.video_file.is_none() && media.season.is_none() {
        media = repositories::media::find_by_season_and_parent_id(&mut connection, 1, media.id)
            .await
            .map_err(|e| {
                error!(
                    "Error fetching media with season 1 and parent_id {}: {}",
                    media.id, e
                );
                Problem::from(ProblemType::InternalServerError(instance.clone()))
            })?
            .ok_or(Problem {
                r#type: "https://developer.mozilla.org/en-US/docs/Web/HTTP/Reference/Status/404"
                    .to_string(),
                title: "Media not found".to_string(),
                status: 400,
                detail: Some(format!(
                    "Media with season 1 and parent_id {} not found",
                    media.id
                )),
                instance: instance.clone(),
            })?;
    }

    if media.video_file.is_none() && media.episode.is_none() {
        media = repositories::media::find_by_episode_and_parent_id(&mut connection, 1, media.id)
            .await
            .map_err(|e| {
                error!(
                    "Error fetching media with episode 1 and parent_id {}: {}",
                    media.id, e
                );
                Problem::from(ProblemType::InternalServerError(instance.clone()))
            })?
            .ok_or(Problem {
                r#type: "https://developer.mozilla.org/en-US/docs/Web/HTTP/Reference/Status/404"
                    .to_string(),
                title: "Media not found".to_string(),
                status: 400,
                detail: Some(format!(
                    "Media with episode 1 and parent_id {} not found",
                    media.id
                )),
                instance: instance.clone(),
            })?;
    }

    if media.video_file.is_none() {
        return Err(Problem {
            r#type: "https://developer.mozilla.org/en-US/docs/Web/HTTP/Reference/Status/404"
                .to_string(),
            title: "Video not found".to_string(),
            status: 400,
            detail: Some(format!("Video for Media with id {media_id} not found")),
            instance,
        });
    }

    let mut components = vec![media.video_file, media.path];
    while let Some(parent_id) = media.parent_id {
        let media = repositories::media::find_by_id(&mut connection, parent_id)
            .await
            .map_err(|e| {
                error!("Error fetching media with id {}: {}", parent_id, e);
                Problem::from(ProblemType::InternalServerError(instance.clone()))
            })?
            .ok_or(Problem {
                r#type: "https://developer.mozilla.org/en-US/docs/Web/HTTP/Reference/Status/404"
                    .to_string(),
                title: "Media not found".to_string(),
                status: 400,
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

    let mut video_file = File::open(&path).await.map_err(|e| {
        error!("Error while opening video file: {}", e);
        Problem::from(ProblemType::InternalServerError(instance.clone()))
    })?;

    let metadata = video_file.metadata().await.map_err(|e| {
        error!("Error reading video files metadata: {}", e);
        Problem::from(ProblemType::InternalServerError(instance.clone()))
    })?;

    let file_size = metadata.len();

    let (start, end) = if let Some(range_header) = headers.get(header::RANGE) {
        let range_str = range_header.to_str().unwrap_or("");
        if !range_str.starts_with("bytes=") {
            return Err(Problem {
                r#type: "https://developer.mozilla.org/en-US/docs/Web/HTTP/Reference/Status/400"
                    .to_string(),
                title: "Invalid Range header".to_string(),
                status: 400,
                instance,
                ..Default::default()
            });
        }

        let mut parts = range_str.trim_start_matches("bytes=").split('-');
        let start = parts
            .next()
            .and_then(|s| s.parse::<u64>().ok())
            .unwrap_or(0);
        let end = parts
            .next()
            .and_then(|e| e.parse::<u64>().ok())
            .unwrap_or(file_size - 1)
            .min(file_size - 1);
        if start > end {
            return Err(Problem {
                r#type: "https://developer.mozilla.org/en-US/docs/Web/HTTP/Reference/Status/416"
                    .to_string(),
                title: "Range not satisfiable".to_string(),
                status: 416,
                instance,
                ..Default::default()
            });
        }

        (start, end)
    } else {
        (0, file_size - 1)
    };

    if start > 0 {
        if let Err(e) = video_file.seek(SeekFrom::Start(start)).await {
            error!("Error seeking video file: {}", e);
            return Err(ProblemType::InternalServerError(instance).into());
        }
    }

    let content_length = end - start + 1;

    let content_type = match path.extension().and_then(|ext| ext.to_str()) {
        Some("mp4") => "video/mp4",
        Some("webm") => "video/webm",
        Some("avi") => "video/x-msvideo",
        Some("mov") => "video/quicktime",
        Some("mkv") => "video/x-matroska",
        _ => "application/octet-stream",
    };

    let limited_reader = video_file.take(content_length);
    let stream = ReaderStream::with_capacity(limited_reader, 65536);

    let mut response_headers = HeaderMap::new();
    response_headers.insert(header::CONTENT_TYPE, content_type.parse().unwrap());
    response_headers.insert(
        header::CONTENT_LENGTH,
        content_length.to_string().parse().unwrap(),
    );
    response_headers.insert(header::ACCEPT_RANGES, "bytes".parse().unwrap());

    if headers.contains_key(header::RANGE) {
        response_headers.insert(
            header::CONTENT_RANGE,
            format!("bytes {start}-{end}/{file_size}").parse().unwrap(),
        );

        return Ok((
            StatusCode::PARTIAL_CONTENT,
            response_headers,
            Body::from_stream(stream),
        ));
    }

    Ok((StatusCode::OK, response_headers, Body::from_stream(stream)))
}
