use crate::r#impl::OpenAPIImpl;
use crate::jobs::ScanLibraryPayload;
use crate::models::Library;
use async_trait::async_trait;
use axum::http::Method;
use axum_extra::extract::{CookieJar, Host};
use effectum::Job;
use tracing::error;
use openapi::apis::libraries::LibrariesPostResponse;
use openapi::models::{LibrariesPost200Response, LibrariesPostRequest};

#[allow(unused_variables)]
#[async_trait]
impl openapi::apis::libraries::Libraries for OpenAPIImpl {
    async fn libraries_post(
        &self,
        method: &Method,
        host: &Host,
        cookies: &CookieJar,
        body: &LibrariesPostRequest,
    ) -> Result<LibrariesPostResponse, ()> {
        let library = match self.repositories.library_repository.create(
            &mut *self.connection.lock().await,
            &mut Library {
                name: body.name.clone(),
                path: body.path.clone(),
                media_type: body.media_type.clone(),
                ..Default::default()
            },
        ).await {
            Ok(library) => library,
            Err(err) => {
                error!("Failed to create library: {:?}", err);
                return Ok(LibrariesPostResponse::Status500_InternalServerError)
            },
        };

        let job_builder = match Job::builder("scan_library").json_payload(&ScanLibraryPayload {
            library_id: library.id.clone(),
        }) {
            Ok(job_builder) => job_builder,
            Err(err) => {
                error!("Failed to create job builder: {:?}", err);
                return Ok(LibrariesPostResponse::Status500_InternalServerError)
            },
        };

        if let Err(err) = job_builder.add_to(&self.queue).await {
            error!("Failed to add job to queue: {:?}", err);
            return Ok(LibrariesPostResponse::Status500_InternalServerError);
        }

        Ok(LibrariesPostResponse::Status200_SingleLibraryItem(
            LibrariesPost200Response {
                id: library.id.to_string(),
                name: library.name,
                path: library.path,
                media_type: library.media_type,
            },
        ))
    }
}
