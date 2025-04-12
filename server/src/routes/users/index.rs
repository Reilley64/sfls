use crate::errors::{Problem, ProblemType};
use crate::middlware::{DbConn, OptionalAuthUser};
use crate::models::InsertableUser;
use crate::repositories;
use axum::response::IntoResponse;
use axum::Json;
use password_auth::generate_hash;
use serde::Deserialize;
use tracing::error;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateUser {
    pub email: String,
    pub password: String,
    pub name: String,
    pub is_admin: bool,
}

pub async fn post(
    DbConn(mut connection): DbConn,
    OptionalAuthUser(auth_user): OptionalAuthUser,
    Json(body): Json<CreateUser>,
) -> Result<impl IntoResponse, Problem> {
    let instance = Some("/users".to_string());

    if auth_user.is_none()
        && repositories::user::count(&mut connection)
            .await
            .map_err(|e| {
                error!("Error counting users: {}", e);
                Problem::from(ProblemType::InternalServerError(instance.clone()))
            })?
            != 0
    {
        return Err(Problem {
            r#type: "https://developer.mozilla.org/en-US/docs/Web/HTTP/Reference/Status/403".into(),
            status: 403,
            title: "Forbidden".to_string(),
            instance,
            ..Default::default()
        });
    }

    let user = InsertableUser {
        created_by: auth_user
            .clone()
            .map_or("SYSTEM".to_string(), |user| user.name),
        updated_by: auth_user
            .clone()
            .map_or("SYSTEM".to_string(), |user| user.name),
        email: body.email,
        password: generate_hash(&body.password),
        name: body.name,
        is_admin: body.is_admin,
    };

    let user = repositories::user::create(&mut connection, &user)
        .await
        .map_err(|e| {
            error!("Error creating user: {}", e);
            Problem::from(ProblemType::InternalServerError(instance.clone()))
        })?;

    Ok(Json(user))
}
