use crate::clients::fanart::FanartService;
use crate::factories::artwork_fetcher::{convert_and_save_image_as_webp, ArtworkFetcher};
use crate::models::{File, FileType};
use crate::repositories;
use crate::state::AppState;
use async_trait::async_trait;
use itertools::Itertools;
use std::path::Path;
use tracing::{info, warn};

pub struct TvShowArtworkFetcher;

#[async_trait]
impl ArtworkFetcher for TvShowArtworkFetcher {
    async fn fetch_artwork(&self, state: AppState, media_id: i64) -> Result<(), anyhow::Error> {
        let mut media = {
            let mut connection = state.pool.get().await?;
            repositories::media::find_by_id(&mut connection, media_id)
                .await
                .map_err(|e| anyhow::Error::msg(format!("Failed to look for media: {e}")))?
                .ok_or(anyhow::Error::msg("Failed to find media".to_string()))?
        };

        let response = FanartService::new()
            .fetch_tv_art(
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

        let file_types = vec![
            FileType::Poster,
            FileType::Logo,
            FileType::Background,
            FileType::Thumbnail,
        ];

        for file_type in file_types {
            if media.files.iter().any(|f| f.type_ == file_type) {
                continue;
            }

            let images: Vec<_> = match file_type {
                FileType::Poster => response.tv_poster.clone(),
                FileType::Logo => response.hdtv_logo.clone(),
                FileType::Thumbnail => response.tv_thumb.clone(),
                FileType::Background => response.show_background.clone(),
                _ => unreachable!(),
            }
            .into_iter()
            .filter(|t| file_type == FileType::Background || t.lang == "en")
            .sorted_by(|a, b| b.likes.cmp(&a.likes))
            .collect();

            match images.first() {
                Some(image) => {
                    let response = reqwest::Client::new()
                        .get(&image.url)
                        .send()
                        .await
                        .map_err(|e| anyhow::Error::msg(format!("Failed to download images: {e}")))?
                        .bytes()
                        .await
                        .map_err(|e| {
                            anyhow::Error::msg(format!("Failed to fetch poster bytes: {e}"))
                        })?;

                    let file_name = match file_type {
                        FileType::Poster => "poster.webp",
                        FileType::Logo => "logo.webp",
                        FileType::Thumbnail => "thumbnail.webp",
                        FileType::Background => "background.webp",
                        _ => unreachable!(),
                    };
                    convert_and_save_image_as_webp(
                        response,
                        &Path::new(&media.path.clone().unwrap()).join(file_name),
                    )?;
                    media.files.as_mut().push(File {
                        type_: file_type,
                        path: file_name.to_string(),
                        blur_hash: None,
                    });
                }
                None => {
                    warn!("No {:?} found for Media {}", file_type, media_id);
                    continue;
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
