use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;
use serde::Serialize;
use std::fmt::{Display, Formatter};

#[derive(Debug, Default, Serialize)]
pub struct Problem {
    pub r#type: String,
    pub title: String,
    pub status: u16,
    pub detail: Option<String>,
    pub instance: Option<String>,
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

pub enum ProblemType {
    Forbidden(Option<String>),
    InternalServerError(Option<String>),
}

impl From<ProblemType> for Problem {
    fn from(problem_type: ProblemType) -> Self {
        match problem_type {
            ProblemType::Forbidden(instance) => Problem {
                r#type: "https://developer.mozilla.org/en-US/docs/Web/HTTP/Reference/Status/403"
                    .to_string(),
                title: "Forbidden".to_string(),
                status: 403,
                detail: None,
                instance,
            },
            ProblemType::InternalServerError(instance) => Problem {
                r#type: "https://developer.mozilla.org/en-US/docs/Web/HTTP/Reference/Status/500"
                    .to_string(),
                title: "Internal server error".to_string(),
                status: 500,
                detail: None,
                instance,
            },
        }
    }
}
