use crate::factories::library_scanner::{remove_empty_self_closing_tags, LibraryScanner};
use crate::jobs::fetch_artwork::{FetchArtwork, FetchArtworkPayload};
use crate::models::{File, FileType, InsertableMedia, Library};
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

        let (video_file, nfo_file, poster_file, logo_file, background_file, thumbnail_file) = {
            let mut video_file = None;
            let mut nfo_file = None;
            let mut poster_file = false;
            let mut logo_file = false;
            let mut background_file = false;
            let mut thumbnail_file = false;

            while let Some(entry) = dir.next_entry().await? {
                if video_file.is_some()
                    && nfo_file.is_some()
                    && poster_file
                    && logo_file
                    && background_file
                    && thumbnail_file
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

                if !poster_file || !logo_file || !background_file || !thumbnail_file {
                    if let Some(file_name) = path.file_name() {
                        match file_name.to_str().unwrap() {
                            "poster.webp" => {
                                poster_file = true;
                                continue;
                            }
                            "logo.webp" => {
                                logo_file = true;
                                continue;
                            }
                            "background.webp" => {
                                background_file = true;
                                continue;
                            }
                            "thumbnail.webp" => {
                                thumbnail_file = true;
                                continue;
                            }
                            _ => {
                                continue;
                            }
                        }
                    }
                }
            }

            (
                video_file,
                nfo_file,
                poster_file,
                logo_file,
                background_file,
                thumbnail_file,
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

        media.files.as_mut().push(File {
            type_: FileType::Video,
            path: video_file.file_name().unwrap().to_str().unwrap().to_string(),
            blur_hash: None,
        });

        if poster_file {
            media.files.as_mut().push(File {
                type_: FileType::Poster,
                path: "poster.webp".to_string(),
                blur_hash: None,
            });
        }

        if logo_file {
            media.files.as_mut().push(File {
                type_: FileType::Logo,
                path: "logo.webp".to_string(),
                blur_hash: None,
            });
        }

        if background_file {
            media.files.as_mut().push(File {
                type_: FileType::Background,
                path: "background.webp".to_string(),
                blur_hash: None,
            });
        }

        if thumbnail_file {
            media.files.as_mut().push(File {
                type_: FileType::Thumbnail,
                path: "thumbnail.webp".to_string(),
                blur_hash: None,
            });
        }

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
