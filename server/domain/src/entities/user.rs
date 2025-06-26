use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct User {
    pub id: i64,
    pub created_at: chrono::NaiveDateTime,
    pub created_by: String,
    pub updated_at: chrono::NaiveDateTime,
    pub updated_by: String,
    pub email: String,
    pub password: String,
    pub name: String,
    pub is_admin: bool,
}