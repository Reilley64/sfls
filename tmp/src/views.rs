use crate::models::Media;
use serde::Serialize;

#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MediaView {
    pub id: String,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
    pub type_: String,
    pub library_id: String,
    pub title: String,
    pub season: Option<i32>,
    pub episode: Option<i32>,
    pub attributes: serde_json::Value,
    pub parent_id: Option<String>,
}

impl From<Media> for MediaView {
    fn from(value: Media) -> Self {
        Self {
            id: value.id.to_string(),
            created_at: value.created_at,
            updated_at: value.updated_at,
            type_: value.type_,
            library_id: value.library_id.to_string(),
            title: value.title,
            season: value.season,
            episode: value.episode,
            attributes: value.attributes,
            parent_id: value.parent_id.map(|id| id.to_string()),
        }
    }
}
