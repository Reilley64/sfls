use crate::jobs::scan_folder::ScanFolderJob;
use crate::jobs::scan_library::ScanLibraryJob;
use application::ports::job::Job;
use application::ports::job_factory::JobFactory;
use application::ports::queue::JobQueue;
use domain::repositories::library::LibraryRepository;
use std::path::PathBuf;
use std::sync::Arc;

#[derive(Clone)]
pub struct JobFactoryImpl {
    job_queue: Arc<dyn JobQueue + Send + Sync>,
    library_repository: Arc<dyn LibraryRepository + Send + Sync>,
}

impl JobFactoryImpl {
    pub fn new(
        job_queue: Arc<dyn JobQueue + Send + Sync>,
        library_repository: Arc<dyn LibraryRepository + Send + Sync>,
    ) -> Self {
        Self {
            job_queue,
            library_repository,
        }
    }
}

impl JobFactory for JobFactoryImpl {
    fn create_scan_library_job(&self, library_id: i64) -> Box<dyn Job> {
        Box::new(ScanLibraryJob {
            job_queue: self.job_queue.clone(),
            job_factory: Arc::new(self.clone()),
            library_repository: self.library_repository.clone(),
            library_id,
        })
    }

    fn create_scan_folder_job(&self, library_id: i64, path: PathBuf) -> Box<dyn Job> {
        Box::new(ScanFolderJob {
            job_queue: self.job_queue.clone(),
            library_repository: self.library_repository.clone(),
            library_id,
            path,
        })
    }
}
