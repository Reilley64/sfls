use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;
use bon::Builder;
use domain::errors::AppError;
use serde::Serialize;
use std::fmt::{Display, Formatter};
use crate::errors::problem_builder::{SetDetail, SetStatus, SetTitle, SetType};

#[derive(Debug, Default, Builder, Serialize)]
#[builder(on(String, into))]
pub struct Problem {
    pub r#type: String,
    pub title: String,
    pub status: u16,
    pub detail: Option<String>,
    #[builder(overwritable)]
    pub instance: Option<String>,
}

impl Problem {
    pub fn to_builder(&self) -> ProblemBuilder<SetDetail<SetStatus<SetTitle<SetType>>>> {
        Problem::builder()
            .r#type(self.r#type.clone())
            .title(self.title.clone())
            .status(self.status.clone())
            .maybe_detail(self.detail.clone())
            .maybe_instance(self.instance.clone())
    }
}

impl Display for Problem {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Problem {{ type: {}, title: {}, status: {} {}{}}}",
            self.r#type,
            self.title,
            self.status,
            self.detail
                .as_ref()
                .map_or(String::new(), |d| format!(", detail: {d}")),
            self.instance
                .as_ref()
                .map_or(String::new(), |i| format!(", instance: {i}"))
        )
    }
}

impl std::error::Error for Problem {}

impl IntoResponse for Problem {
    fn into_response(self) -> Response {
        let status = StatusCode::from_u16(self.status).unwrap_or(StatusCode::INTERNAL_SERVER_ERROR);

        (
            status,
            [("Content-Type", "application/problem+json")],
            Json(self),
        )
            .into_response()
    }
}

impl From<AppError> for Problem {
    fn from(value: AppError) -> Self {
        match value {
            AppError::ValidationError(message) => Problem::builder()
                .r#type("https://developer.mozilla.org/en-US/docs/Web/HTTP/Reference/Status/400")
                .title("Validation error")
                .status(400)
                .detail(message)
                .build(),
            AppError::AnyhowError(_e) => Problem::builder()
                .r#type("https://developer.mozilla.org/en-US/docs/Web/HTTP/Reference/Status/500")
                .title("Internal server error")
                .status(500)
                .build(),
            AppError::Error(_e) => Problem::builder()
                .r#type("https://developer.mozilla.org/en-US/docs/Web/HTTP/Reference/Status/500")
                .title("Internal server error")
                .status(500)
                .build(),
        }
    }
}
