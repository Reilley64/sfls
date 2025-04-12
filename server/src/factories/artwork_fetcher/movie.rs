use crate::clients::fanart::FanartService;
use crate::factories::artwork_fetcher::{convert_and_save_image_as_webp, ArtworkFetcher};
use crate::repositories;
use crate::state::AppState;
use async_trait::async_trait;
use std::path::Path;
use tracing::{info, warn};

pub struct MovieArtworkFetcher;

#[async_trait]
impl ArtworkFetcher for MovieArtworkFetcher {
    async fn fetch_artwork(&self, state: AppState, media_id: i64) -> Result<(), anyhow::Error> {
        let mut media = {
            let mut connection = state.pool.get().await?;
            repositories::media::find_by_id(&mut connection, media_id)
                .await
                .map_err(|e| anyhow::Error::msg(format!("Failed to look for media: {e}")))?
                .ok_or(anyhow::Error::msg("Failed to find media".to_string()))?
        };

        let response = FanartService::new()
            .fetch_movie_art(
                media
                    .attributes
                    .get("nfoId")
                    .ok_or(anyhow::Error::msg(
                        "Media attributes has no nfoId".to_string(),
                    ))?
                    .to_string(),
            )
            .await
            .map_err(|e| anyhow::Error::msg(format!("Failed to get artwork: {e}")))?;

        if media.path.is_none() {
            return Err(anyhow::Error::msg("Media path not set".to_string()));
        }

        if media.poster_file.is_none() {
            let mut posters: Vec<_> = response
                .movie_poster
                .into_iter()
                .filter(|p| p.lang == "en")
                .collect();
            posters.sort_by(|a, b| b.likes.cmp(&a.likes));

            match posters.first() {
                Some(poster) => {
                    let response = reqwest::Client::new()
                        .get(&poster.url)
                        .send()
                        .await
                        .map_err(|e| anyhow::Error::msg(format!("Failed to download poster: {e}")))?
                        .bytes()
                        .await
                        .map_err(|e| {
                            anyhow::Error::msg(format!("Failed to fetch poster bytes: {e}"))
                        })?;

                    let file_name = "poster.webp";
                    convert_and_save_image_as_webp(
                        response,
                        &Path::new(&media.path.clone().unwrap()).join(file_name),
                    )?;
                    media.poster_file = Some(file_name.to_string());
                }
                None => {
                    warn!("No poster found for Media {}", media_id);
                }
            }
        }

        if media.thumbnail_file.is_none() {
            let mut thumbnails: Vec<_> = response
                .movie_thumb
                .into_iter()
                .filter(|t| t.lang == "en")
                .collect();
            thumbnails.sort_by(|a, b| b.likes.cmp(&a.likes));

            match thumbnails.first() {
                Some(thumbnail) => {
                    let response = reqwest::Client::new()
                        .get(&thumbnail.url)
                        .send()
                        .await
                        .map_err(|e| {
                            anyhow::Error::msg(format!("Failed to download thumbnail: {e}"))
                        })?
                        .bytes()
                        .await
                        .map_err(|e| {
                            anyhow::Error::msg(format!("Failed to fetch thumbnail bytes: {e}"))
                        })?;

                    let file_name = "thumbnail.webp";
                    convert_and_save_image_as_webp(
                        response,
                        &Path::new(&media.path.clone().unwrap()).join(file_name),
                    )?;
                    media.thumbnail_file = Some(file_name.to_string());
                }
                None => {
                    warn!("No thumbnail found for Media {}", media_id);
                }
            }
        }

        if media.fanart_file.is_none() {
            let mut fanarts: Vec<_> = response.movie_background;
            fanarts.sort_by(|a, b| b.likes.cmp(&a.likes));

            match fanarts.first() {
                Some(fanart) => {
                    let response = reqwest::Client::new()
                        .get(&fanart.url)
                        .send()
                        .await
                        .map_err(|e| anyhow::Error::msg(format!("Failed to download fanart: {e}")))?
                        .bytes()
                        .await
                        .map_err(|e| {
                            anyhow::Error::msg(format!("Failed to fetch fanart bytes: {e}"))
                        })?;

                    let file_name = "fanart.webp";
                    convert_and_save_image_as_webp(
                        response,
                        &Path::new(&media.path.clone().unwrap()).join(file_name),
                    )?;
                    media.fanart_file = Some(file_name.to_string());
                }
                None => {
                    warn!("No fanart found for Media {}", media_id);
                }
            }
        }

        {
            let mut connection = state.pool.get().await?;
            repositories::media::update(&mut connection, &media)
                .await
                .map_err(|e| {
                    anyhow::Error::msg(format!("Failed to update media thumbnail path: {e}"))
                })?;
        }

        info!("Finished downloading artwork for media: {}", media_id);
        Ok(())
    }
}
