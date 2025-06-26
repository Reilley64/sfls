use crate::jobs::Job;
use crate::repositories;
use crate::state::AppState;
use async_trait::async_trait;
use std::path::Path;
use tracing::info;

pub struct ScanFolderPayload {
    pub library_id: i64,
    pub folder_path: String,
}

impl ScanFolderPayload {
    pub fn new(library_id: i64, folder_path: String) -> Self {
        Self {
            library_id,
            folder_path,
        }
    }
}

pub struct ScanFolder {
    pub state: AppState,
    pub payload: ScanFolderPayload,
}

impl ScanFolder {
    pub fn new(state: AppState, payload: ScanFolderPayload) -> Self {
        Self { state, payload }
    }
}

#[async_trait]
impl Job for ScanFolder {
    async fn run(&self) -> Result<(), anyhow::Error> {
        info!("Scanning folder: {}", self.payload.folder_path);

        let library = {
            let mut connection = self.state.pool.get().await?;

            match repositories::library::find_by_id(&mut connection, self.payload.library_id)
                .await?
            {
                Some(library) => library,
                None => {
                    return Err(anyhow::Error::msg(format!(
                        "Library with id {} not found",
                        self.payload.library_id
                    )));
                }
            }
        };

        self.state
            .scanner_factory
            .get_scanner(&library.media_type)
            .ok_or(anyhow::Error::msg("Unknown media type".to_string()))?
            .scan_folder(
                self.state.clone(),
                library,
                Path::new(&self.payload.folder_path),
            )
            .await?;

        info!("Finished scanning folder: {}", self.payload.folder_path);
        Ok(())
    }
}
