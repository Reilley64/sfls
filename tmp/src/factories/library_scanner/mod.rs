use crate::models::Library;
use crate::state::AppState;
use async_trait::async_trait;
use regex::Regex;
use std::collections::HashMap;
use std::path::Path;

pub mod movie;
pub mod tvshow;

#[async_trait]
pub trait LibraryScanner {
    async fn scan_folder(
        &self,
        state: AppState,
        library: Library,
        folder_path: &Path,
    ) -> Result<(), anyhow::Error>;
}

pub struct ScannerFactory {
    pub scanners: HashMap<String, Box<dyn LibraryScanner + Send + Sync>>,
}

impl ScannerFactory {
    pub fn new() -> Self {
        let mut scanners = HashMap::new();
        scanners.insert(
            "movie".to_string(),
            Box::new(movie::MovieScanner) as Box<dyn LibraryScanner + Send + Sync>,
        );
        scanners.insert(
            "tvshow".to_string(),
            Box::new(tvshow::TvShowScanner) as Box<dyn LibraryScanner + Send + Sync>,
        );
        Self { scanners }
    }

    pub fn get_scanner(&self, library_type: &str) -> Option<&(dyn LibraryScanner + Send + Sync)> {
        self.scanners
            .get(library_type)
            .map(std::convert::AsRef::as_ref)
    }
}

impl Default for ScannerFactory {
    fn default() -> Self {
        Self::new()
    }
}

pub fn remove_empty_self_closing_tags(xml: &str) -> String {
    let re = Regex::new(r"<([a-zA-Z0-9_:-]+)\s*/>").unwrap();
    re.replace_all(xml, "").to_string()
}
