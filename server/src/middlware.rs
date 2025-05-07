use crate::errors::{Problem, ProblemType};
use crate::models::User;
use crate::repositories;
use crate::state::AppState;
use axum::extract::{FromRef, FromRequestParts};
use axum::http::request::Parts;
use deadpool::managed::Object;
use diesel_async::pooled_connection::AsyncDieselConnectionManager;
use diesel_async::AsyncPgConnection;
use jsonwebtoken::{DecodingKey, Validation};
use serde::{Deserialize, Serialize};
use tracing::error;

pub struct DbConn(pub Object<AsyncDieselConnectionManager<AsyncPgConnection>>);

impl<S> FromRequestParts<S> for DbConn
where
    AppState: FromRef<S>,
    S: Send + Sync,
{
    type Rejection = (axum::http::StatusCode, String);

    async fn from_request_parts(_parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let state = AppState::from_ref(state);

        let connection = state.pool.get().await.map_err(|e| {
            error!("Error getting connection from pool: {}", e);

            (
                axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to get connection from pool".to_string(),
            )
        })?;

        Ok(Self(connection))
    }
}

#[derive(Serialize, Deserialize)]
pub struct Claims {
    pub sub: i64,
    pub exp: usize,
    pub iat: usize,
}

pub type AuthUser = User;

impl<S> FromRequestParts<S> for AuthUser
where
    AppState: FromRef<S>,
    S: Send + Sync,
{
    type Rejection = Problem;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let state = AppState::from_ref(state);

        let authorization = match parts.headers.get("Authorization") {
            Some(header) => header.to_str().unwrap(),
            None => {
                return Err(Problem {
                    r#type:
                        "https://developer.mozilla.org/en-US/docs/Web/HTTP/Reference/Status/401"
                            .to_string(),
                    status: 401,
                    title: "Unauthorized".to_string(),
                    detail: Some("Authorization header is missing".to_string()),
                    ..Default::default()
                });
            }
        };

        let parts: Vec<&str> = authorization.split(' ').collect();
        if parts.len() != 2 {
            return Err(Problem {
                r#type: "https://developer.mozilla.org/en-US/docs/Web/HTTP/Reference/Status/401"
                    .to_string(),
                status: 401,
                title: "Unauthorized".to_string(),
                detail: Some("Invalid authorization header".to_string()),
                ..Default::default()
            });
        }

        let claims = jsonwebtoken::decode::<Claims>(
            parts[1],
            &DecodingKey::from_secret(&state.secret),
            &Validation::default(),
        )
        .map_err(|e| {
            error!("Error decoding token: {}", e);
            Problem {
                r#type: "https://developer.mozilla.org/en-US/docs/Web/HTTP/Reference/Status/401"
                    .to_string(),
                status: 401,
                title: "Unauthorized".to_string(),
                detail: Some("Invalid authorization header".to_string()),
                ..Default::default()
            }
        })?;

        let user = {
            let mut connection = state.pool.get().await.map_err(|e| {
                error!("Error getting connection from pool: {}", e);
                Problem {
                    r#type:
                        "https://developer.mozilla.org/en-US/docs/Web/HTTP/Reference/Status/500"
                            .to_string(),
                    status: 500,
                    title: "Internal Server Error".to_string(),
                    detail: Some("Failed to get connection from pool".to_string()),
                    ..Default::default()
                }
            })?;
            match repositories::user::find_by_id(&mut connection, claims.claims.sub)
                .await
                .map_err(|e| {
                    error!("Error finding user by id: {}", e);
                    Problem::from(ProblemType::InternalServerError(None))
                })? {
                Some(user) => user,
                None => return Err(ProblemType::InternalServerError(None).into()),
            }
        };

        Ok(user)
    }
}

pub struct OptionalAuthUser(pub Option<AuthUser>);

impl<S> FromRequestParts<S> for OptionalAuthUser
where
    AppState: FromRef<S>,
    S: Send + Sync,
{
    type Rejection = Problem;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        match AuthUser::from_request_parts(parts, state).await {
            Ok(user) => Ok(Self(Some(user))),
            Err(_e) => Ok(Self(None)),
        }
    }
}
