use serde::{Deserialize, Serialize};
use crate::entities::media_type::MediaType;

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Library {
    pub id: i64,
    pub created_at: chrono::NaiveDateTime,
    pub created_by: String,
    pub updated_at: chrono::NaiveDateTime,
    pub updated_by: String,
    pub name: String,
    pub path: String,
    pub media_type: MediaType,
}
