use crate::factories::library_scanner::{remove_empty_self_closing_tags, LibraryScanner};
use crate::jobs::fetch_artwork::{FetchArtwork, FetchArtworkPayload};
use crate::jobs::scan_season_folder::{ScanSeasonFolder, ScanSeasonFolderPayload};
use crate::models::{InsertableMedia, Library};
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

        let (nfo_file, poster_file, thumbnail_file, fanart_file, season_folders) = {
            let mut nfo_file = None;
            let mut poster_file = false;
            let mut thumbnail_file = false;
            let mut fanart_file = false;
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

                if !poster_file && entry.path().file_name().is_some_and(|f| f == "poster.webp") {
                    poster_file = true;
                    continue;
                }

                if !thumbnail_file
                    && entry
                        .path()
                        .file_name()
                        .is_some_and(|f| f == "thumbnail.webp")
                {
                    thumbnail_file = true;
                    continue;
                }

                if !fanart_file && entry.path().file_name().is_some_and(|f| f == "fanart.webp") {
                    fanart_file = true;
                }
            }

            (
                nfo_file,
                poster_file,
                thumbnail_file,
                fanart_file,
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
