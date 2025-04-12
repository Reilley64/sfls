use crate::jobs::JobContext;
use crate::models::{Library, Media};
use crate::nfo::NFO;
use crate::scanners::{LibraryScanner, remove_empty_self_closing_tags};
use async_trait::async_trait;
use effectum::Error;
use serde_json::json;
use std::fs;
use std::io::Read;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use tracing::{debug, warn};

pub struct TvShowScanner;

#[async_trait]
impl LibraryScanner for TvShowScanner {
    async fn scan_folder(
        &self,
        context: Arc<JobContext>,
        library: Library,
        folder_path: &Path,
    ) -> Result<(), Error> {
        let entries: Vec<_> = fs::read_dir(folder_path)
            .map_err(|e| Error::InvalidJobState("Failed to read dir".to_string()))?
            .filter_map(Result::ok)
            .collect();

        let files: Vec<PathBuf> = entries
            .iter()
            .filter_map(|entry| {
                if !entry.path().is_file() {
                    return None;
                }

                Some(entry.path())
            })
            .collect();

        let nfo_file = files.iter().find(|f| f.extension().unwrap() == "nfo");

        if nfo_file.is_none() {
            warn!("No nfo file found in folder: {:?}", folder_path);
            return Ok(());
        }

        let nfo_file = nfo_file.unwrap();
        let nfo_string =
            remove_empty_self_closing_tags(fs::read_to_string(nfo_file).unwrap().as_str());

        let nfo: NFO = quick_xml::de::from_str(nfo_string.as_str()).unwrap();

        let mut media = Media::from(nfo);
        media.type_ = library.media_type.clone();
        media.library_id = library.id.clone();
        media.path = folder_path.to_str().unwrap().to_string();

        let parent = context
            .repositories
            .media_repository
            .create(&mut *context.connection.lock().await, &mut media)
            .await
            .unwrap();

        let season_folders: Vec<PathBuf> = entries
            .into_iter()
            .filter_map(|entry| {
                if !entry
                    .file_name()
                    .to_str()
                    .unwrap()
                    .to_lowercase()
                    .starts_with("season ")
                {
                    return None;
                }

                Some(entry.path())
            })
            .collect();

        for season_folder in season_folders {
            let entries: Vec<_> = fs::read_dir(season_folder)
                .map_err(|e| Error::InvalidJobState("Failed to read dir".to_string()))?
                .filter_map(Result::ok)
                .collect();

            let episode_nfo_files = entries.iter().filter_map(|entry| {
                if entry.path().extension().is_some() && entry.path().extension().unwrap() == "nfo"
                {
                    return Some(entry.path());
                }

                None
            });

            for episode_nfo_file in episode_nfo_files {
                let video_file = entries.iter().find(|entry| {
                    let actual = entry.file_name().to_str().unwrap().replace(
                        format!(".{}", entry.path().extension().unwrap().to_str().unwrap())
                            .as_str(),
                        "",
                    );
                    let expectation = episode_nfo_file
                        .file_name()
                        .unwrap()
                        .to_str()
                        .unwrap()
                        .replace(".nfo", "");

                    if actual != expectation {
                        return false;
                    }

                    if let Ok(mut open) = fs::File::open(entry.path()) {
                        let mut buffer = [0; 8192];
                        if let Ok(bytes_read) = open.read(&mut buffer[..]) {
                            if bytes_read > 0 {
                                if let Some(kind) = infer::get(&buffer[..bytes_read]) {
                                    if kind.mime_type().starts_with("video/") {
                                        return true;
                                    }
                                }
                            }
                        }
                    }

                    false
                });

                if video_file.is_none() {
                    warn!("No video file found for nfo file: {:?}", episode_nfo_file);
                    continue;
                }

                let video_file = video_file.unwrap();

                let nfo_string =
                    remove_empty_self_closing_tags(fs::read_to_string(episode_nfo_file).unwrap().as_str());

                let nfo: NFO = quick_xml::de::from_str(nfo_string.as_str()).unwrap();

                let mut media = Media::from(nfo);
                media.type_ = library.media_type.clone();
                media.library_id = library.id.clone();
                media.path = folder_path.to_str().unwrap().to_string();
                media.video_file = Some(
                    video_file
                        .file_name()
                        .to_str()
                        .unwrap()
                        .to_string(),
                );
                media.parent_id = Some(parent.id.clone());

                context
                    .repositories
                    .media_repository
                    .create(&mut *context.connection.lock().await, &mut media)
                    .await
                    .map_err(|e| Error::InvalidJobState("Failed to save episode".to_string()))?;
            }
        }

        Ok(())
    }
}
