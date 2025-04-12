use std::str::FromStr;
use crate::r#impl::OpenAPIImpl;
use crate::repositories::media::MediaCriteria;
use async_trait::async_trait;
use axum::http::Method;
use axum_extra::extract::{CookieJar, Host};
use serde_json::json;
use openapi::apis::media::{MediaGetResponse, MediaMediaIdGetResponse};
use openapi::models::{MediaMediaIdGetPathParams, Problem};

#[allow(unused_variables)]
#[async_trait]
impl openapi::apis::media::Media for OpenAPIImpl {
    async fn media_get(
        &self,
        method: &Method,
        host: &Host,
        cookies: &CookieJar,
    ) -> Result<MediaGetResponse, ()> {
        let instance = "/media".to_string();

        match self
            .repositories
            .media_repository
            .find_all(&mut *self.connection.lock().await, MediaCriteria {})
            .await
        {
            Ok(media) => Ok(MediaGetResponse::Status200_ListOfMediaItems(
                media
                    .iter()
                    .map(|media| openapi::models::Media {
                        id: media.id.clone().to_string(),
                        created_at: media.created_at.clone(),
                        updated_at: media.updated_at.clone(),
                        title: media.title.clone(),
                        season: media.season.clone(),
                        episode: media.episode.clone(),
                        attributes: serde_json::Value::from_str(media.attributes.clone()).into_iter().collect(),
                    })
                    .collect(),
            )),
            Err(err) => Ok(MediaGetResponse::Status500_UnexpectedProblem(
                Problem {
                    r#type: Some(
                        "https://developer.mozilla.org/en-US/docs/Web/HTTP/Reference/Status/500"
                            .to_string(),
                    ),
                    title: "Internal server error".to_string(),
                    status: 500,
                    detail: None,
                    instance: Some(instance),
                },
            )),
        }
    }

    async fn media_media_id_get(
        &self,
        method: &Method,
        host: &Host,
        cookies: &CookieJar,
        path_params: &MediaMediaIdGetPathParams,
    ) -> Result<MediaMediaIdGetResponse, ()> {
        let instance = format!("/media/{}", path_params.media_id);

        let media_id = match path_params.media_id.parse::<i64>() {
            Ok(media_id) => media_id,
            Err(_) => return Ok(MediaMediaIdGetResponse::Status400_UnexpectedProblem(
                Problem {
                    r#type: Some(
                        "https://developer.mozilla.org/en-US/docs/Web/HTTP/Reference/Status/400"
                            .to_string(),
                    ),
                    title: "media_id is not a valid id".to_string(),
                    status: 400,
                    detail: Some(format!(
                        "media_id {} is not a valid id",
                        path_params.media_id
                    )),
                    instance: Some(instance),
                },
            )),
        };

        match self
            .repositories
            .media_repository
            .find_by_id(&mut *self.connection.lock().await, media_id)
            .await
        {
            Ok(Some(media)) => Ok(MediaMediaIdGetResponse::Status200_SingleMediaItem(
                openapi::models::Media {
                    id: media.id.to_string(),
                },
            )),
            Ok(None) => Ok(MediaMediaIdGetResponse::Status404_UnexpectedProblem(
                Problem {
                    r#type: Some(
                        "https://developer.mozilla.org/en-US/docs/Web/HTTP/Reference/Status/400"
                            .to_string(),
                    ),
                    title: "Media not found".to_string(),
                    status: 400,
                    detail: Some(format!("Media with id {} not found", path_params.media_id)),
                    instance: Some(instance),
                },
            )),
            Err(err) => Ok(MediaMediaIdGetResponse::Status500_UnexpectedProblem(
                Problem {
                    r#type: Some(
                        "https://developer.mozilla.org/en-US/docs/Web/HTTP/Reference/Status/500"
                            .to_string(),
                    ),
                    title: "Internal server error".to_string(),
                    status: 500,
                    detail: None,
                    instance: Some(instance),
                },
            )),
        }
    }
}
