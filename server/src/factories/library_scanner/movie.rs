use crate::factories::library_scanner::{remove_empty_self_closing_tags, LibraryScanner};
use crate::jobs::fetch_artwork::{FetchArtwork, FetchArtworkPayload};
use crate::models::{InsertableMedia, Library};
use crate::nfo::Nfo;
use crate::repositories;
use crate::state::AppState;
use async_trait::async_trait;
use std::path::Path;
use tokio::fs;
use tokio::io::AsyncReadExt;

pub struct MovieScanner;

#[async_trait]
impl LibraryScanner for MovieScanner {
    async fn scan_folder(
        &self,
        state: AppState,
        library: Library,
        folder_path: &Path,
    ) -> Result<(), anyhow::Error> {
        let mut dir = fs::read_dir(folder_path).await?;

        let (video_file, nfo_file, poster_file, thumbnail_file, fanart_file) = {
            let mut video_file = None;
            let mut nfo_file = None;
            let mut poster_file = false;
            let mut thumbnail_file = false;
            let mut fanart_file = false;

            while let Some(entry) = dir.next_entry().await? {
                if video_file.is_some()
                    && nfo_file.is_some()
                    && poster_file
                    && thumbnail_file
                    && fanart_file
                {
                    break;
                }

                let path = entry.path();

                if nfo_file.is_none() {
                    if let Some(extension) = path.extension() {
                        if extension == "nfo" {
                            nfo_file = Some(path);
                            continue;
                        }
                    }
                }

                if video_file.is_none() {
                    if let Ok(mut open) = fs::File::open(path.clone()).await {
                        let mut buffer = [0; 8192];
                        if let Ok(bytes_read) = open.read(&mut buffer[..]).await {
                            if bytes_read > 0 {
                                if let Some(kind) = infer::get(&buffer[..bytes_read]) {
                                    if kind.mime_type().starts_with("video/") {
                                        video_file = Some(path);
                                        continue;
                                    }
                                }
                            }
                        }
                    }
                }

                if !poster_file {
                    if let Some(file_name) = path.file_name() {
                        if file_name == "poster.webp" {
                            poster_file = true;
                            continue;
                        }
                    }
                }

                if !thumbnail_file {
                    if let Some(file_name) = path.file_name() {
                        if file_name == "thumbnail.webp" {
                            thumbnail_file = true;
                            continue;
                        }
                    }
                }

                if !fanart_file {
                    if let Some(file_name) = path.file_name() {
                        if file_name == "fanart.webp" {
                            fanart_file = true;
                        }
                    }
                }
            }

            (
                video_file,
                nfo_file,
                poster_file,
                thumbnail_file,
                fanart_file,
            )
        };
        
        let Some(video_file) = video_file else {
            return Err(anyhow::Error::msg(
                "No video file found in folder".to_string(),
            ));
        };
        
        let Some(nfo_file) = nfo_file else {
            return Err(anyhow::Error::msg(
                "No nfo file found in folder".to_string(),
            ));       
        };

        let nfo_string =
            remove_empty_self_closing_tags(fs::read_to_string(nfo_file).await?.as_str());
        let nfo: Nfo = quick_xml::de::from_str(nfo_string.as_str())?;

        let mut media = InsertableMedia::from(nfo);
        media.type_.clone_from(&library.media_type);
        media.library_id = library.id;
        media.path = Some(folder_path.to_str().unwrap().to_string());
        media.video_file = Some(
            video_file
                .file_name()
                .unwrap()
                .to_str()
                .unwrap()
                .to_string(),
        );
        media.video_file_size = Some(
            tokio::fs::metadata(&video_file)
                .await
                .map_err(|_e| anyhow::Error::msg("Error getting file size".to_string()))?
                .len() as i64,
        );
        media.poster_file = if poster_file {
            Some("poster.webp".to_string())
        } else {
            None
        };
        media.thumbnail_file = if thumbnail_file {
            Some("thumbnail.webp".to_string())
        } else {
            None
        };
        media.fanart_file = if fanart_file {
            Some("fanart.webp".to_string())
        } else {
            None
        };

        let media = {
            let mut connection = state.pool.get().await?;
            repositories::media::create(&mut connection, &media).await?
        };

        state.queue.send(Box::new(FetchArtwork::new(
            state.clone(),
            FetchArtworkPayload::new(media.id),
        )))?;

        Ok(())
    }
}
