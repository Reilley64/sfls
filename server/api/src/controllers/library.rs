use crate::errors::Problem;
use crate::models::library::CreateLibraryRequest;
use application::commands::command::Command;
use application::commands::library::create::{CreateLibraryCommand, CreateLibraryCommandInput};
use axum::extract::State;
use axum::response::IntoResponse;
use axum::Json;
use domain::entities::user::User;
use infrastructure::state::AppState;
use std::path::PathBuf;

pub async fn create_library_handler(
    State(state): State<AppState>,
    Json(body): Json<CreateLibraryRequest>,
) -> Result<impl IntoResponse, Problem> {
    let instance = "/libraries".to_string();

    if body.name.trim().is_empty() {
        return Err(Problem::builder()
            .r#type(
                "https://developer.mozilla.org/en-US/docs/Web/HTTP/Reference/Status/400"
                    .to_string(),
            )
            .title("Validation error".to_string())
            .status(400)
            .detail("name must not be empty".to_string())
            .instance(instance.clone())
            .build());
    }

    let library = CreateLibraryCommand::new(
        state.job_queue,
        state.job_factory,
        state.repositories.library_repository,
    )
    .handle(CreateLibraryCommandInput {
        user: User::default(),
        name: body.name.clone(),
        path: PathBuf::from(body.path.clone()),
        media_type: body.media_type.clone(),
    })
    .await
    .map_err(|e| {
        Problem::from(e)
            .to_builder()
            .instance(instance.clone())
            .build()
    })?;

    Ok(Json(library))
}
