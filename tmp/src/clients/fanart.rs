use serde::{Deserialize, Serialize};

const FANART_BASE_URL: &str = "https://webservice.fanart.tv/v3";
const FANART_API_KEY: &str = "8a541d9b51adeedb584a32eed079c432";
const FANART_CLIENT_KEY: &str = "4a041ef55797cc4af9f3023270b45dda";

#[derive(Debug, Serialize, Deserialize)]
pub struct FanartMovieResponse {
    #[serde(rename = "tmdb_id")]
    pub id: String,
    pub name: String,
    #[serde(rename = "hdmovielogo", default)]
    pub hdmovie_logo: Vec<FanartImage>,
    #[serde(rename = "moviedisc", default)]
    pub movie_disc: Vec<FanartImage>,
    #[serde(rename = "movielogo", default)]
    pub movie_logo: Vec<FanartImage>,
    #[serde(rename = "movieposter", default)]
    pub movie_poster: Vec<FanartImage>,
    #[serde(rename = "hdmovieclearart", default)]
    pub hdmovie_clearart: Vec<FanartImage>,
    #[serde(rename = "movieart", default)]
    pub movie_art: Vec<FanartImage>,
    #[serde(rename = "moviebackground", default)]
    pub movie_background: Vec<FanartImage>,
    #[serde(rename = "moviebanner", default)]
    pub movie_banner: Vec<FanartImage>,
    #[serde(rename = "moviethumb", default)]
    pub movie_thumb: Vec<FanartImage>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FanartTVResponse {
    #[serde(rename = "thetvdb_id")]
    pub id: String,
    pub name: String,
    #[serde(rename = "clearlogo", default)]
    pub clear_logo: Vec<FanartImage>,
    #[serde(rename = "hdtvlogo", default)]
    pub hdtv_logo: Vec<FanartImage>,
    #[serde(rename = "clearart", default)]
    pub clear_art: Vec<FanartImage>,
    #[serde(rename = "showbackground", default)]
    pub show_background: Vec<FanartImage>,
    #[serde(rename = "tvthumb", default)]
    pub tv_thumb: Vec<FanartImage>,
    #[serde(rename = "seasonposter", default)]
    pub season_poster: Vec<FanartImage>,
    #[serde(rename = "seasonthumb", default)]
    pub season_thumb: Vec<FanartImage>,
    #[serde(rename = "hdclearart", default)]
    pub hdclear_art: Vec<FanartImage>,
    #[serde(rename = "tvbanner", default)]
    pub tv_banner: Vec<FanartImage>,
    #[serde(rename = "characterart", default)]
    pub character_art: Vec<FanartImage>,
    #[serde(rename = "tvposter", default)]
    pub tv_poster: Vec<FanartImage>,
    #[serde(rename = "seasonbanner", default)]
    pub season_banner: Vec<FanartImage>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FanartImage {
    pub id: String,
    pub url: String,
    pub lang: String,
    pub likes: String,
    pub disc: Option<String>,
    pub disc_type: Option<String>,
    pub season: Option<String>,
}

pub struct FanartService {
    client: reqwest::Client,
}

impl FanartService {
    pub fn new() -> Self {
        Self {
            client: reqwest::Client::new(),
        }
    }

    pub async fn fetch_movie_art(&self, id: String) -> anyhow::Result<FanartMovieResponse> {
        let response = self
            .client
            .get(format!(
                "{FANART_BASE_URL}/movies/{id}?api_key={FANART_API_KEY}&client_key={FANART_CLIENT_KEY}"
            ))
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(anyhow::Error::msg(format!(
                "Failed to fetch movie artwork: {}",
                response.text().await?
            )));
        }

        let body = response.text().await?;
        serde_json::from_str(&body).map_err(|e| {
            anyhow::Error::msg(format!(
                "Failed to parse movie artwork: {e}, for body: {body}"
            ))
        })
    }

    pub async fn fetch_tv_art(&self, id: String) -> anyhow::Result<FanartTVResponse> {
        let response = self
            .client
            .get(format!(
                "{FANART_BASE_URL}/tv/{id}?api_key={FANART_API_KEY}&client_key={FANART_CLIENT_KEY}"
            ))
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(anyhow::Error::msg(format!(
                "Failed to fetch movie artwork: {}",
                response.text().await?
            )));
        }

        let body = response.text().await?;
        serde_json::from_str(&body).map_err(|e| {
            anyhow::Error::msg(format!(
                "Failed to parse movie artwork: {e}, for body: {body}"
            ))
        })
    }
}
