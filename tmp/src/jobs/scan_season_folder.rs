use crate::factories::library_scanner::remove_empty_self_closing_tags;
use crate::jobs::Job;
use crate::models::{File, FileType, InsertableMedia};
use crate::nfo::Nfo;
use crate::repositories;
use crate::state::AppState;
use async_trait::async_trait;
use std::collections::HashMap;
use std::path::PathBuf;
use tokio::fs::DirEntry;
use tokio::io::AsyncReadExt;
use tracing::{info, warn};

pub struct ScanSeasonFolderPayload {
    pub media_id: i64,
    pub season_folder: DirEntry,
}

impl ScanSeasonFolderPayload {
    pub fn new(media_id: i64, season_folder: DirEntry) -> Self {
        Self {
            media_id,
            season_folder,
        }
    }
}

pub struct ScanSeasonFolder {
    pub state: AppState,
    pub payload: ScanSeasonFolderPayload,
}

impl ScanSeasonFolder {
    pub fn new(state: AppState, payload: ScanSeasonFolderPayload) -> Self {
        Self { state, payload }
    }
}

#[async_trait]
impl Job for ScanSeasonFolder {
    async fn run(&self) -> Result<(), anyhow::Error> {
        info!("Scanning season folder: {:?}", self.payload.season_folder);

        let season_folder = self.payload.season_folder.path().clone();

        let season_file_name = season_folder.file_name().unwrap().to_str().unwrap();

        let season: i32 = season_file_name
            .split(' ')
            .next_back()
            .ok_or(anyhow::Error::msg(
                "Failed to get last element of season file name".to_string(),
            ))?
            .trim_start_matches('0')
            .parse()?;

        let parent = {
            let mut connection = self.state.pool.get().await?;
            match repositories::media::find_by_id(&mut connection, self.payload.media_id).await? {
                Some(media) => media,
                None => {
                    return Err(anyhow::Error::msg(format!(
                        "Media with id {} not found",
                        self.payload.media_id
                    )));
                }
            }
        };

        let media = InsertableMedia {
            type_: parent.type_.clone(),
            library_id: parent.library_id,
            path: Some(season_file_name.to_string()),
            title: season_file_name.to_string(),
            season: Some(season),
            parent_id: Some(parent.id),
            ..Default::default()
        };

        let parent = {
            let mut connection = self.state.pool.get().await?;

            let existing = repositories::media::find_by_path_and_parent_id(
                &mut connection,
                media.path.clone(),
                media.parent_id,
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

        let mut dir = tokio::fs::read_dir(season_folder).await?;

        let mut map: HashMap<String, (Option<PathBuf>, Option<PathBuf>, Option<PathBuf>)> =
            HashMap::new();

        while let Some(entry) = dir.next_entry().await? {
            let path = entry.path();

            if !path.is_file() {
                continue;
            }

            let file_name_without_extension = entry.file_name().to_str().unwrap().replace(
                format!(
                    ".{}",
                    if let Some(extension) = path.extension() {
                        extension.to_str().unwrap()
                    } else {
                        warn!("File found without extension {:?}", entry);
                        continue;
                    }
                )
                .as_str(),
                "",
            );

            if let Some(extension) = path.extension() {
                if extension == "nfo" {
                    let item = map.get(&file_name_without_extension.clone());
                    map.insert(
                        file_name_without_extension.clone(),
                        (
                            Some(path),
                            item.and_then(|i| i.1.clone()),
                            item.and_then(|i| i.2.clone()),
                        ),
                    );
                    continue;
                }
            }

            if let Ok(mut open) = tokio::fs::File::open(path.clone()).await {
                let mut buffer = [0; 8192];
                if let Ok(bytes_read) = open.read(&mut buffer[..]).await {
                    if bytes_read > 0 {
                        if let Some(kind) = infer::get(&buffer[..bytes_read]) {
                            if kind.mime_type().starts_with("video/") {
                                let item = map.get(&file_name_without_extension.clone());
                                map.insert(
                                    file_name_without_extension.clone(),
                                    (
                                        item.and_then(|i| i.0.clone()),
                                        Some(path),
                                        item.and_then(|i| i.2.clone()),
                                    ),
                                );
                                continue;
                            }
                        }
                    }
                }
            }

            if let Some(file_name) = entry.file_name().to_str() {
                if file_name.ends_with("-thumb.jpg") {
                    let file_name_without_extension = file_name.replace("-thumb.jpg", "");
                    let item = map.get(&file_name_without_extension.clone());
                    map.insert(
                        file_name_without_extension.clone(),
                        (
                            item.and_then(|i| i.0.clone()),
                            item.and_then(|i| i.1.clone()),
                            Some(path),
                        ),
                    );
                }
            }
        }

        for (file_name, (nfo_file, video_file, thumbnail_file)) in map {
            let Some(nfo_file) = nfo_file else {
                warn!("No nfo file found for entry: {:?}", file_name);
                continue;
            };

            let Some(video_file) = video_file else {
                warn!("No video file found for entry: {:?}", file_name);
                continue;
            };

            let Some(thumbnail_file) = thumbnail_file else {
                warn!("No thumbnail file found for entry: {:?}", file_name);
                continue;
            };

            let nfo_string =
                remove_empty_self_closing_tags(tokio::fs::read_to_string(nfo_file).await?.as_str());
            let nfo: Nfo = quick_xml::de::from_str(nfo_string.as_str())?;

            let mut media = InsertableMedia::from(nfo);
            media.type_.clone_from(&parent.type_);
            media.library_id = parent.library_id;
            media.parent_id = Some(parent.id);

            media.files.as_mut().push(File {
                type_: FileType::Video,
                path: video_file.to_str().unwrap().to_string(),
                blur_hash: None,
            });

            media.files.as_mut().push(File {
                type_: FileType::Poster,
                path: thumbnail_file.to_str().unwrap().to_string(),
                blur_hash: None,
            });

            {
                let mut connection = self.state.pool.get().await?;
                repositories::media::create(&mut connection, &media).await?;
            }
        }

        info!(
            "Finished scanning season folder: {:?}",
            self.payload.season_folder
        );
        Ok(())
    }
}
