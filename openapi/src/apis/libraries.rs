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
pub enum LibrariesPostResponse {
    /// Single library item
    Status200_SingleLibraryItem
    (models::LibrariesPost200Response)
    ,
    /// Internal server error
    Status500_InternalServerError
}


/// Libraries
#[async_trait]
#[allow(clippy::ptr_arg)]
pub trait Libraries<E: std::fmt::Debug + Send + Sync + 'static = ()>: super::ErrorHandler<E> {
    /// Create a library item.
    ///
    /// LibrariesPost - POST /libraries
    async fn libraries_post(
    &self,
    method: &Method,
    host: &Host,
    cookies: &CookieJar,
            body: &models::LibrariesPostRequest,
    ) -> Result<LibrariesPostResponse, E>;
}
