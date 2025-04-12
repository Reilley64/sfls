pub mod movie;
pub mod tvshow;

use crate::jobs::JobContext;
use crate::models::Library;
use async_trait::async_trait;
use effectum::Error;
use regex::Regex;
use std::path::Path;
use std::sync::Arc;

#[async_trait]
pub trait LibraryScanner {
    async fn scan_folder(
        &self,
        context: Arc<JobContext>,
        library: Library,
        folder_path: &Path,
    ) -> Result<(), Error>;
}

pub fn remove_empty_self_closing_tags(xml: &str) -> String {
    let re = Regex::new(r"<([a-zA-Z0-9_:-]+)\s*/>").unwrap();
    re.replace_all(xml, "").to_string()
}

