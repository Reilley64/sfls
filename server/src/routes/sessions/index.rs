use crate::errors::{Problem, ProblemType};
use crate::middlware::{Claims, DbConn};
use crate::repositories;
use crate::state::AppState;
use axum::extract::State;
use axum::response::IntoResponse;
use axum::Json;
use chrono::Utc;
use jsonwebtoken::{EncodingKey, Header};
use password_auth::{verify_password, VerifyError};
use serde::{Deserialize, Serialize};
use tracing::error;

#[derive(Deserialize)]
pub struct Payload {
    pub email: String,
    pub password: String,
}

pub async fn post(
    State(state): State<AppState>,
    DbConn(mut connection): DbConn,
    Json(body): Json<Payload>,
) -> Result<impl IntoResponse, Problem> {
    #[derive(Serialize)]
    struct AuthResponse {
        token: String,
    }

    let instance = Some("/sessions".to_string());

    let user = repositories::user::find_by_email(&mut connection, body.email)
        .await
        .map_err(|e| {
            error!("Error calling user::find_by_email: {}", e);
            Problem::from(ProblemType::InternalServerError(instance.clone()))
        })?
        .ok_or(Problem::from(ProblemType::Forbidden(instance.clone())))?;

    if let Err(e) = verify_password(&body.password, &user.password) {
        return match e {
            VerifyError::Parse(e) => {
                error!("Error parsing password: {}", e);
                Err(ProblemType::InternalServerError(instance).into())
            }
            VerifyError::PasswordInvalid => Err(ProblemType::Forbidden(instance).into()),
        };
    }

    let claims = Claims {
        sub: user.id,
        exp: usize::try_from(
            Utc::now()
                .checked_add_signed(chrono::Duration::days(1))
                .ok_or(Problem::from(ProblemType::InternalServerError(
                    instance.clone(),
                )))?
                .timestamp(),
        )
        .map_err(|e| {
            error!("Error converting exp: {}", e);
            Problem::from(ProblemType::InternalServerError(instance.clone()))
        })?,
        iat: usize::try_from(Utc::now().timestamp()).map_err(|e| {
            error!("Error converting iat: {}", e);
            Problem::from(ProblemType::InternalServerError(instance.clone()))
        })?,
    };

    let token = jsonwebtoken::encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(&state.secret),
    )
    .map_err(|e| {
        error!("Error encoding token: {}", e);
        Problem::from(ProblemType::InternalServerError(instance.clone()))
    })?;

    Ok(Json(AuthResponse { token }))
}
