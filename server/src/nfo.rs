use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct Nfo {
    pub title: String,
    #[serde(rename = "originaltitle")]
    pub original_title: Option<String>,
    #[serde(rename = "sorttitle")]
    pub sort_title: Option<String>,
    pub season: Option<i32>,
    pub episode: Option<i32>,
    pub ratings: Option<Ratings>,
    pub rating: Option<f64>,
    #[serde(rename = "criticrating")]
    pub critic_rating: Option<i32>,
    #[serde(rename = "userrating")]
    pub user_rating: Option<i32>,
    #[serde(rename = "top250")]
    pub top_250: Option<i32>,
    pub outline: Option<String>,
    pub plot: Option<String>,
    pub tagline: Option<String>,
    pub runtime: Option<i32>,
    pub thumb: Option<Thumb>,
    pub fanart: Option<Fanart>,
    pub mpaa: Option<String>,
    pub id: Option<i64>,
    pub genre: Option<Vec<String>>,
    pub country: Option<String>,
    pub status: Option<String>,
    // pub credits: String,
    // pub director: String,
    pub premiered: Option<chrono::NaiveDate>,
    pub end_date: Option<chrono::NaiveDate>,
    pub year: Option<i32>,
    pub studio: Option<String>,
    pub trailer: Option<String>,
    #[serde(rename = "fileinfo")]
    pub file_info: Option<FileInfo>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Ratings {
    #[serde(rename = "$value")]
    pub content: Vec<Rating>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Rating {
    #[serde(rename = "@name")]
    pub name: String,
    #[serde(rename = "@max")]
    pub max: i32,
    #[serde(rename = "@default", default)]
    pub default: bool,
    pub value: f64,
    pub votes: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Thumb {
    #[serde(rename = "@aspect")]
    pub aspect: Option<String>,
    #[serde(rename = "@preview")]
    pub preview: String,
    #[serde(rename = "$value")]
    pub content: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Fanart {
    pub thumb: Thumb,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileInfo {
    #[serde(rename = "streamdetails")]
    pub stream_details: StreamDetails,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StreamDetails {
    pub video: Video,
    pub audio: Audio,
    #[serde(default)]
    pub subtitle: Option<Vec<Subtitle>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Video {
    pub aspect: String,
    pub bitrate: String,
    pub codec: String,
    pub framerate: String,
    pub height: String,
    #[serde(rename = "scantype")]
    pub scan_type: String,
    pub width: String,
    pub duration: String,
    #[serde(rename = "durationinseconds")]
    pub duration_in_seconds: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Audio {
    pub bitrate: String,
    pub channels: String,
    pub codec: String,
    pub language: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Subtitle {
    pub language: Option<String>,
}
