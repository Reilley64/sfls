use crate::errors::{Problem, ProblemType};
use crate::jobs::scan_library::{ScanLibrary, ScanLibraryPayload};
use crate::middlware::{AuthUser, DbConn};
use crate::models::InsertableLibrary;
use crate::repositories;
use crate::state::AppState;
use axum::extract::State;
use axum::response::IntoResponse;
use axum::Json;
use serde::Deserialize;
use tracing::error;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateLibrary {
    name: String,
    path: String,
    media_type: String,
}

pub async fn post(
    State(state): State<AppState>,
    DbConn(mut connection): DbConn,
    auth_user: AuthUser,
    Json(body): Json<CreateLibrary>,
) -> Result<impl IntoResponse, Problem> {
    let instance = Some("/libraries".to_string());

    let library = repositories::library::create(
        &mut connection,
        &InsertableLibrary {
            created_by: auth_user.name.clone(),
            updated_by: auth_user.name.clone(),
            name: body.name.clone(),
            path: body.path.clone(),
            media_type: body.media_type.clone().to_string(),
        },
    )
    .await
    .map_err(|e| {
        error!("Error creating library: {:?}", e);
        Problem::from(ProblemType::InternalServerError(instance.clone()))
    })?;

    if let Err(e) = state.queue.send(Box::new(ScanLibrary::new(
        state.clone(),
        ScanLibraryPayload::new(library.id),
    ))) {
        error!("Failed to add job to queue: {:?}", e);
        return Err(ProblemType::InternalServerError(instance).into());
    }

    Ok(Json(library))
}
