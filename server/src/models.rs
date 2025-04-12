use crate::nfo::Nfo;
use diesel::{AsChangeset, Insertable, Queryable, QueryableByName, Selectable};
use serde::Serialize;
use serde_json::json;

#[derive(Debug, Default, Serialize, Queryable, Selectable, Insertable)]
#[serde(rename_all = "camelCase")]
#[diesel(table_name = crate::schema::libraries)]
pub struct InsertableLibrary {
    pub created_by: String,
    pub updated_by: String,
    pub name: String,
    pub path: String,
    pub media_type: String,
}

#[derive(Debug, Default, Serialize, Queryable, Selectable)]
#[serde(rename_all = "camelCase")]
#[diesel(table_name = crate::schema::libraries)]
pub struct Library {
    pub id: i64,
    pub created_at: chrono::NaiveDateTime,
    pub created_by: String,
    pub updated_at: chrono::NaiveDateTime,
    pub updated_by: String,
    pub name: String,
    pub path: String,
    pub media_type: String,
}

#[derive(Debug, Default, Insertable)]
#[diesel(table_name = crate::schema::media)]
pub struct InsertableMedia {
    pub type_: String,
    pub library_id: i64,
    pub path: Option<String>,
    pub video_file: Option<String>,
    pub video_file_size: Option<i64>,
    pub poster_file: Option<String>,
    pub thumbnail_file: Option<String>,
    pub fanart_file: Option<String>,
    pub title: String,
    pub season: Option<i32>,
    pub episode: Option<i32>,
    pub attributes: serde_json::Value,
    pub parent_id: Option<i64>,
}

impl From<Nfo> for InsertableMedia {
    fn from(value: Nfo) -> Self {
        Self {
            title: value.sort_title.clone().unwrap_or(value.title.clone()),
            season: value.season,
            episode: value.episode,
            attributes: json!({
                "title": value.title.clone(),
                "originalTitle": value.original_title.clone(),
                "sortTitle": value.sort_title.clone(),
                "ratings": value.ratings.clone().map(|r| r.content),
                "rating": value.rating.clone(),
                "criticRating": value.critic_rating.clone(),
                "userRating": value.user_rating.clone(),
                "top250": value.top_250.clone(),
                "outline": value.outline.clone(),
                "plot": value.plot.clone(),
                "tagline": value.tagline.clone(),
                "runtime": value.runtime.clone(),
                "mpaa": value.mpaa.clone(),
                "nfoId": value.id.clone(),
                "genre": value.genre.clone(),
                "country": value.country.clone(),
                "status": value.status.clone(),
                "premiered": value.premiered.clone(),
                "endDate": value.end_date.clone(),
                "year": value.year.clone(),
                "studio": value.studio.clone(),
                "trailer": value.trailer.clone(),
                "fileInfo": value.file_info.clone(),
            }),
            ..Default::default()
        }
    }
}

#[derive(Debug, Default, Queryable, QueryableByName, Selectable, AsChangeset)]
#[diesel(table_name = crate::schema::media)]
pub struct Media {
    pub id: i64,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
    pub type_: String,
    pub library_id: i64,
    pub path: Option<String>,
    pub video_file: Option<String>,
    pub video_file_size: Option<i64>,
    pub poster_file: Option<String>,
    pub thumbnail_file: Option<String>,
    pub fanart_file: Option<String>,
    pub title: String,
    pub season: Option<i32>,
    pub episode: Option<i32>,
    pub attributes: serde_json::Value,
    pub parent_id: Option<i64>,
}

impl Media {
    pub fn apply(&mut self, insertable: &InsertableMedia) {
        self.type_.clone_from(&insertable.type_);
        self.library_id = insertable.library_id;
        self.path.clone_from(&insertable.path);
        self.video_file.clone_from(&insertable.video_file);
        self.thumbnail_file.clone_from(&insertable.thumbnail_file);
        self.fanart_file.clone_from(&insertable.fanart_file);
        self.title.clone_from(&insertable.title);
        self.season = insertable.season;
        self.episode = insertable.episode;
        self.attributes = insertable.attributes.clone();
        self.parent_id = insertable.parent_id;
    }
}

#[derive(Debug, Default, Insertable)]
#[diesel(table_name = crate::schema::users)]
pub struct InsertableUser {
    pub created_by: String,
    pub updated_by: String,
    pub email: String,
    pub password: String,
    pub name: String,
    pub is_admin: bool,
}

#[derive(Debug, Clone, Default, Serialize, Queryable, Selectable, AsChangeset)]
#[serde(rename_all = "camelCase")]
#[diesel(table_name = crate::schema::users)]
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

#[derive(Debug, Clone, Default, Insertable)]
#[diesel(table_name = crate::schema::history)]
pub struct InsertableHistory {
    pub media_id: i64,
    pub user_id: i64,
    pub position: i64,
}

#[derive(Debug, Default, Serialize, Queryable, Selectable)]
#[serde(rename_all = "camelCase")]
#[diesel(table_name = crate::schema::history)]
pub struct History {
    pub id: i64,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
    pub media_id: i64,
    pub user_id: i64,
    pub position: i64,
}
