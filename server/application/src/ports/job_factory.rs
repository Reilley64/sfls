use std::path::PathBuf;
use crate::ports::job::Job;

pub trait JobFactory {
    fn create_scan_library_job(&self, library_id: i64) -> Box<dyn Job>;
    
    fn create_scan_folder_job(&self, library_id: i64, path: PathBuf) -> Box<dyn Job>;
}
