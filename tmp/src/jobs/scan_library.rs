use crate::jobs::scan_folder::{ScanFolder, ScanFolderPayload};
use crate::jobs::Job;
use crate::repositories;
use crate::state::AppState;
use async_trait::async_trait;
use std::path::Path;
use tokio::fs;
use tracing::info;

pub struct ScanLibraryPayload {
    library_id: i64,
}

impl ScanLibraryPayload {
    pub fn new(library_id: i64) -> Self {
        Self { library_id }
    }
}

pub struct ScanLibrary {
    state: AppState,
    payload: ScanLibraryPayload,
}

impl ScanLibrary {
    pub fn new(state: AppState, payload: ScanLibraryPayload) -> Self {
        Self { state, payload }
    }
}

#[async_trait]
impl Job for ScanLibrary {
    async fn run(&self) -> Result<(), anyhow::Error> {
        info!("Scanning library: {}", self.payload.library_id);

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

        let path = Path::new(&library.path);

        if !path.is_dir() {
            return Err(anyhow::Error::msg("Path is not a directory"));
        }

        let mut dir = fs::read_dir(path).await?;

        while let Some(entry) = dir.next_entry().await? {
            if !entry.path().is_dir() {
                continue;
            }

            self.state.queue.send(Box::new(ScanFolder::new(
                self.state.clone(),
                ScanFolderPayload::new(library.id, entry.path().to_str().unwrap().to_string()),
            )))?;
        }

        info!("Finished scanning library: {}", self.payload.library_id);
        Ok(())
    }
}
