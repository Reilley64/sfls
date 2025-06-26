use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Media {
    pub id: i64,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
    pub type_: String,
    pub library_id: i64,
    pub path: Option<String>,
    pub title: String,
    pub season: Option<i32>,
    pub episode: Option<i32>,
    pub files: serde_json::Value,
    pub attributes: serde_json::Value,
    pub parent_id: Option<i64>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct MediaCriteria {
    pub types: Option<Vec<String>>,
    pub title: Option<String>,
    pub library_id: Option<i64>,
    pub parent_id: Option<i64>,
    pub order_by: Option<MediaCriteriaOrder>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MediaCriteriaOrder {
    Title,
    Random,
}

