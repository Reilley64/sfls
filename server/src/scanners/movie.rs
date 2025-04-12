use crate::jobs::JobContext;
use crate::models::{Library, Media};
use crate::nfo::NFO;
use crate::scanners::{LibraryScanner, remove_empty_self_closing_tags};
use async_trait::async_trait;
use effectum::Error;
use serde_json::json;
use std::fs;
use std::io::{BufReader, Read};
use std::path::{Path, PathBuf};
use std::sync::Arc;
use tracing::{debug, warn};

pub struct MovieScanner;

#[async_trait]
impl LibraryScanner for MovieScanner {
    async fn scan_folder(
        &self,
        context: Arc<JobContext>,
        library: Library,
        folder_path: &Path,
    ) -> Result<(), Error> {
        let files: Vec<PathBuf> = fs::read_dir(folder_path)
            .unwrap()
            .filter_map(|entry| {
                entry.ok().and_then(|e| {
                    if e.path().is_file() {
                        Some(e.path())
                    } else {
                        None
                    }
                })
            })
            .collect();

        let (video_file, nfo_file) = {
            let mut video_file = None;
            let mut nfo_file = None;

            for file in files.iter() {
                if nfo_file.is_none() {
                    if let Some(extension) = file.extension() {
                        if extension == "nfo" {
                            nfo_file = Some(file);
                        }
                    }
                }

                if video_file.is_none() {
                    if let Ok(mut open) = fs::File::open(file) {
                        let mut buffer = [0; 8192];
                        if let Ok(bytes_read) = open.read(&mut buffer[..]) {
                            if bytes_read > 0 {
                                if let Some(kind) = infer::get(&buffer[..bytes_read]) {
                                    if kind.mime_type().starts_with("video/") {
                                        video_file = Some(file);
                                    }
                                }
                            }
                        }
                    }
                }

                if video_file.is_some() && nfo_file.is_some() {
                    break;
                }
            }

            (video_file, nfo_file)
        };

        if video_file.is_none() || nfo_file.is_none() {
            warn!("No video or nfo file found in folder: {:?}", folder_path);
            return Ok(());
        }

        let video_file = video_file.unwrap();
        let nfo_file = nfo_file.unwrap();

        let nfo_string = remove_empty_self_closing_tags(fs::read_to_string(nfo_file).unwrap().as_str());

        let nfo: NFO = quick_xml::de::from_str(nfo_string.as_str()).unwrap();

        let mut media = Media::from(nfo);
        media.type_ = library.media_type.clone();
        media.library_id = library.id.clone();
        media.path = folder_path.to_str().unwrap().to_string();
        media.video_file = Some(
            video_file
                .file_name()
                .unwrap()
                .to_str()
                .unwrap()
                .to_string(),
        );

        context
            .repositories
            .media_repository
            .create(&mut *context.connection.lock().await, &mut media)
            .await
            .unwrap();

        Ok(())
    }
}
