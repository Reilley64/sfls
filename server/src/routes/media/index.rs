use crate::errors::{Problem, ProblemType};
use crate::middlware::DbConn;
use crate::repositories;
use crate::repositories::media::MediaCriteria;
use crate::views::MediaView;
use axum::response::IntoResponse;
use axum::Json;
use axum_extra::extract::Query;
use serde::Deserialize;
use tracing::error;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct QueryParams {
    types: Option<Vec<String>>,
    title: Option<String>,
    library_id: Option<String>,
    parent_id: Option<String>,
}

pub async fn get(
    DbConn(mut connection): DbConn,
    query_params: Query<QueryParams>,
) -> Result<impl IntoResponse, Problem> {
    let instance = Some("/media".to_string());

    Ok(Json(
        repositories::media::find_all(
            &mut connection,
            MediaCriteria {
                types: query_params.types.clone(),
                title: query_params.title.clone(),
                library_id: query_params
                    .library_id
                    .as_ref()
                    .map_or(Ok(None), |id| id.parse::<i64>().map(Some))
                    .map_err(|_e| Problem {
                        r#type:
                            "https://developer.mozilla.org/en-US/docs/Web/HTTP/Reference/Status/400"
                                .to_string(),
                        title: "library_id is not a valid id".to_string(),
                        status: 400,
                        detail: Some(format!(
                            "library_id {} is not a valid id",
                            query_params.library_id.clone().unwrap()
                        )),
                        instance: instance.clone(),
                    })?,
                parent_id: query_params
                    .parent_id
                    .as_ref()
                    .map_or(Ok(None), |id| id.parse::<i64>().map(Some))
                    .map_err(|_e| Problem {
                        r#type:
                            "https://developer.mozilla.org/en-US/docs/Web/HTTP/Reference/Status/400"
                                .to_string(),
                        title: "parent_id is not a valid id".to_string(),
                        status: 400,
                        detail: Some(format!(
                            "parent_id {} is not a valid id",
                            query_params.library_id.clone().unwrap()
                        )),
                        instance: instance.clone(),
                    })?,
            },
        )
        .await
        .map_err(|e| {
            error!("Error calling media::find_all: {}", e);
            Problem::from(ProblemType::InternalServerError(instance))
        })?
        .into_iter()
        .map(MediaView::from)
        .collect::<Vec<_>>(),
    ))
}
