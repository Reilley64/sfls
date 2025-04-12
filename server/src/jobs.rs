use crate::models::{Library, Media};
use crate::nfo::NFO;
use crate::r#impl::Repositories;
use crate::scanners::movie::MovieScanner;
use crate::scanners::tvshow::TvShowScanner;
use crate::scanners::LibraryScanner;
use diesel::SqliteConnection;
use effectum::{Error, Job, Queue, RunningJob};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::collections::HashMap;
use std::fmt::{Debug, Formatter};
use std::io::{BufReader, Read};
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::{fmt, fs};
use tokio::sync::Mutex;
use tracing::{error, info, warn};

pub struct ScannerFactory {
    scanners: HashMap<String, Box<dyn LibraryScanner + Send + Sync>>,
}

impl ScannerFactory {
    pub fn new() -> Self {
        let mut scanners = HashMap::new();
        scanners.insert("movie".to_string(), Box::new(MovieScanner) as Box<dyn LibraryScanner + Send + Sync>);
        scanners.insert("tvshow".to_string(), Box::new(TvShowScanner) as Box<dyn LibraryScanner + Send + Sync>);

        Self { scanners }
    }

    pub fn get_scanner(&self, library_type: &str) -> Option<&(dyn LibraryScanner + Send + Sync)> {
        self.scanners.get(library_type).map(|s| s.as_ref())
    }
}

pub struct JobContext {
    pub connection: Arc<Mutex<SqliteConnection>>,
    pub repositories: Arc<Repositories>,
    pub queue: Arc<Queue>,
    pub scanner_factory: ScannerFactory,
}

impl Debug for JobContext {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_struct("JobContext")
            .field("pool", &"Arc<DatabasePool>")
            .field("repositories", &"Arc<Repositories>")
            .field("queue", &"Arc<Queue>")
            .field("scanner_factory", &"ScannerFactory")
            .finish()
    }
}

#[derive(Serialize, Deserialize)]
pub struct ScanLibraryPayload {
    pub library_id: i64,
}

pub async fn scan_library_job(job: RunningJob, context: Arc<JobContext>) -> Result<(), Error> {
    let payload: ScanLibraryPayload = job.json_payload()?;

    let library = match context
        .repositories
        .library_repository
        .find_by_id(&mut *context.connection.lock().await, payload.library_id)
        .await
        .unwrap()
    {
        Some(library) => library,
        None => return Err(Error::InvalidJobState("Library not found".to_string())),
    };

    let path = Path::new(&library.path);

    if !path.is_dir() {
        panic!("Path is not a directory");
    }

    let entries = fs::read_dir(path)
        .map_err(|e| Error::InvalidJobState(format!("Failed to read directory: {}", e)))?;

    let folders: Vec<PathBuf> = entries
        .filter_map(|entry| {
            entry.ok().and_then(|e| {
                if e.path().is_dir() {
                    Some(e.path().to_path_buf())
                } else {
                    None
                }
            })
        })
        .collect();

    for folder in folders {
        let job_builder = match Job::builder("scan_folder").json_payload(&ScanFolderPayload {
            library_id: library.id.clone(),
            folder_path: folder.to_str().unwrap().to_string(),
        }) {
            Ok(job_builder) => job_builder,
            Err(e) => {
                error!("Failed to create job builder: {:?}", e);
                return Err(Error::InvalidJobState(format!("Failed to create job builder: {}", e)));
            },
        };

        if let Err(e) = job_builder.add_to(&context.queue).await {
            error!("Failed to add job to queue: {:?}", e);
            return Err(Error::InvalidJobState(format!("Failed to add job to queue: {}", e)));
        }
    }

    Ok(())
}

#[derive(Serialize, Deserialize)]
pub struct ScanFolderPayload {
    pub library_id: i64,
    pub folder_path: String,
}

pub async fn scan_folder_job(job: RunningJob, context: Arc<JobContext>) -> Result<(), Error> {
    let payload: ScanFolderPayload = job.json_payload()?;

    let library = match context
        .repositories
        .library_repository
        .find_by_id(&mut *context.connection.lock().await, payload.library_id)
        .await
        .unwrap()
    {
        Some(library) => library,
        None => return Err(Error::InvalidJobState("Library not found".to_string())),
    };

    let scanner = context.scanner_factory.get_scanner(&library.media_type).ok_or(Error::InvalidJobState("Unknown media type".to_string()))?;

    scanner.scan_folder(context.clone(), library, Path::new(&payload.folder_path)).await
}
