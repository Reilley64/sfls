use async_trait::async_trait;
use axum::extract::*;
use axum_extra::extract::{CookieJar, Host};
use bytes::Bytes;
use http::Method;
use serde::{Deserialize, Serialize};

use crate::{models, types::*};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum MediaGetResponse {
    /// List of media items
    Status200_ListOfMediaItems
    (Vec<models::Media>)
    ,
    /// Unexpected problem
    Status500_UnexpectedProblem
    (models::Problem)
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum MediaMediaIdGetResponse {
    /// Single media item
    Status200_SingleMediaItem
    (models::Media)
    ,
    /// Unexpected problem
    Status400_UnexpectedProblem
    (models::Problem)
    ,
    /// Unexpected problem
    Status404_UnexpectedProblem
    (models::Problem)
    ,
    /// Unexpected problem
    Status500_UnexpectedProblem
    (models::Problem)
}


/// Media
#[async_trait]
#[allow(clippy::ptr_arg)]
pub trait Media<E: std::fmt::Debug + Send + Sync + 'static = ()>: super::ErrorHandler<E> {
    /// Returns a list of media items.
    ///
    /// MediaGet - GET /media
    async fn media_get(
    &self,
    method: &Method,
    host: &Host,
    cookies: &CookieJar,
    ) -> Result<MediaGetResponse, E>;

    /// Returns a single media item.
    ///
    /// MediaMediaIdGet - GET /media/{mediaId}
    async fn media_media_id_get(
    &self,
    method: &Method,
    host: &Host,
    cookies: &CookieJar,
      path_params: &models::MediaMediaIdGetPathParams,
    ) -> Result<MediaMediaIdGetResponse, E>;
}
