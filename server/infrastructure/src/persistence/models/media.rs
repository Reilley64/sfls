use diesel::{AsChangeset, Queryable, QueryableByName, Selectable};
use diesel_json::Json;
use crate::persistence::models::file::File;

#[derive(Debug, Default, Queryable, QueryableByName, Selectable, AsChangeset)]
#[diesel(table_name = crate::schema::media)]
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
    pub files: Json<Vec<File>>,
    pub attributes: serde_json::Value,
    pub parent_id: Option<i64>,
}