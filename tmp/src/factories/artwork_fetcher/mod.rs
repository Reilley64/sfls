use crate::state::AppState;
use async_trait::async_trait;
use axum::body::Bytes;
use image::{ImageFormat, ImageReader};
use std::collections::HashMap;
use std::io::Cursor;
use std::path::PathBuf;

pub mod movie;
pub mod tvshow;

#[async_trait]
pub trait ArtworkFetcher {
    async fn fetch_artwork(&self, context: AppState, media_id: i64) -> Result<(), anyhow::Error>;
}

pub struct ArtworkFetcherFactory {
    pub artwork_fetchers: HashMap<String, Box<dyn ArtworkFetcher + Send + Sync>>,
}

impl ArtworkFetcherFactory {
    pub fn new() -> Self {
        let mut artwork_fetchers = HashMap::new();
        artwork_fetchers.insert(
            "movie".to_string(),
            Box::new(movie::MovieArtworkFetcher) as Box<dyn ArtworkFetcher + Send + Sync>,
        );
        artwork_fetchers.insert(
            "tvshow".to_string(),
            Box::new(tvshow::TvShowArtworkFetcher) as Box<dyn ArtworkFetcher + Send + Sync>,
        );
        Self { artwork_fetchers }
    }
}

impl Default for ArtworkFetcherFactory {
    fn default() -> Self {
        Self::new()
    }
}

pub fn convert_and_save_image_as_webp(
    img_bytes: Bytes,
    path: &PathBuf,
) -> Result<(), anyhow::Error> {
    let img = ImageReader::new(Cursor::new(&img_bytes))
        .with_guessed_format()
        .map_err(|e| anyhow::Error::msg(format!("Failed to guess thumbnail format: {e}")))?
        .decode()
        .map_err(|e| anyhow::Error::msg(format!("Failed to decode thumbnail: {e}")))?;
    img.save_with_format(path, ImageFormat::WebP)
        .map_err(|e| anyhow::Error::msg(format!("Failed to save thumbnail: {e}")))?;
    Ok(())
}
