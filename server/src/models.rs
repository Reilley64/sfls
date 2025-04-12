use std::str::FromStr;
use crate::nfo::NFO;
use diesel::{Insertable, Queryable, Selectable};
use serde_json::json;

#[derive(Debug, Default, Queryable, Selectable, Insertable)]
#[diesel(table_name = crate::schema::libraries)]
pub struct Library {
    pub id: i64,
    pub created_at: String,
    pub updated_at: String,
    pub name: String,
    pub path: String,
    pub media_type: String,
}

#[derive(Debug, Default, Queryable, Selectable, Insertable)]
#[diesel(table_name = crate::schema::media)]
pub struct Media {
    pub id: i64,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
    pub type_: String,
    pub nfo_id: Option<i64>,
    pub library_id: i64,
    pub path: String,
    pub video_file: Option<String>,
    pub title: String,
    pub season: Option<i32>,
    pub episode: Option<i32>,
    pub attributes: String,
    pub parent_id: Option<i64>,
}

impl From<NFO> for Media {
    fn from(value: NFO) -> Self {
        Self {
            nfo_id: value.id.clone(),
            title: value.sort_title.clone().unwrap_or(value.title.clone()),
            season: value.season.clone(),
            episode: value.episode.clone(),
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
                "thumb": value.thumb.clone(),
                "fanart": value.fanart.clone().map(|f| f.thumb),
                "mpaa": value.mpaa.clone(),
                "genre": value.genre.clone(),
                "country": value.country.clone(),
                "status": value.status.clone(),
                "premiered": value.premiered.clone(),
                "endDate": value.end_date.clone(),
                "year": value.year.clone(),
                "studio": value.studio.clone(),
                "trailer": value.trailer.clone(),
                "fileInfo": value.file_info.clone(),
            })
            .to_string(),
            ..Default::default()
        }
    }
}

impl Into<openapi::models::Media> for Media {
    fn into(self) -> openapi::models::Media {
        openapi::models::Media {
            id: self.id.clone().to_string(),
            created_at: self.created_at.clone().and_utc(),
            updated_at: self.updated_at.clone().and_utc(),
            title: self.title.clone(),
            season: self.season.clone(),
            episode: self.episode.clone(),
            attributes: serde_json::from_str::<std::collections::HashMap<String, openapi::types::Object>>(&self.attributes)
                .unwrap_or_default(),
        }
    }
}
