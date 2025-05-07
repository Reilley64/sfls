use crate::factories::library_scanner::{remove_empty_self_closing_tags, LibraryScanner};
use crate::jobs::fetch_artwork::{FetchArtwork, FetchArtworkPayload};
use crate::jobs::scan_season_folder::{ScanSeasonFolder, ScanSeasonFolderPayload};
use crate::models::{File, FileType, InsertableMedia, Library};
use crate::nfo::Nfo;
use crate::repositories;
use crate::state::AppState;
use async_trait::async_trait;
use std::path::Path;
use tokio::fs;

pub struct TvShowScanner;

#[async_trait]
impl LibraryScanner for TvShowScanner {
    async fn scan_folder(
        &self,
        state: AppState,
        library: Library,
        folder_path: &Path,
    ) -> Result<(), anyhow::Error> {
        let mut dir = fs::read_dir(folder_path).await?;

        let (nfo_file, poster_file, logo_file, background_file, thumbnail_file, season_folders) = {
            let mut nfo_file = None;
            let mut poster_file = false;
            let mut logo_file = false;
            let mut background_file = false;
            let mut thumbnail_file = false;
            let mut season_folders = Vec::new();

            while let Some(entry) = dir.next_entry().await? {
                if entry.path().is_dir() {
                    if entry
                        .file_name()
                        .to_str()
                        .unwrap()
                        .to_lowercase()
                        .starts_with("season ")
                    {
                        season_folders.push(entry);
                    }

                    continue;
                }

                if nfo_file.is_none() && entry.path().extension().is_some_and(|f| f == "nfo") {
                    nfo_file = Some(entry);
                    continue;
                }

                if !poster_file || !logo_file || !background_file || !thumbnail_file {
                    if let Some(file_name) = entry.path().file_name() {
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
                nfo_file,
                poster_file,
                logo_file,
                background_file,
                thumbnail_file,
                season_folders,
            )
        };

        let Some(nfo_file) = nfo_file else {
            return Err(anyhow::Error::msg(
                "No nfo file found in folder".to_string(),
            ));
        };

        let nfo_string =
            remove_empty_self_closing_tags(fs::read_to_string(nfo_file.path()).await?.as_str());
        let nfo: Nfo = quick_xml::de::from_str(nfo_string.as_str())?;

        let mut media = InsertableMedia::from(nfo);
        media.type_.clone_from(&library.media_type);
        media.library_id = library.id;
        media.path = Some(folder_path.to_str().unwrap().to_string());

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

        let parent = {
            let mut connection = state.pool.get().await?;

            let existing = repositories::media::find_by_path_and_parent_id(
                &mut connection,
                media.path.clone(),
                None,
            )
            .await?;

            match existing {
                Some(mut existing) => {
                    existing.apply(&media);
                    repositories::media::update(&mut connection, &existing).await?
                }
                None => repositories::media::create(&mut connection, &media).await?,
            }
        };

        state.queue.send(Box::new(FetchArtwork::new(
            state.clone(),
            FetchArtworkPayload::new(parent.id),
        )))?;

        for season_folder in season_folders {
            state.queue.send(Box::new(ScanSeasonFolder::new(
                state.clone(),
                ScanSeasonFolderPayload::new(parent.id, season_folder),
            )))?;
        }

        Ok(())
    }
}
