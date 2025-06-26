pub mod di;

pub mod jobs {
    pub mod factory;
    pub mod scan_folder;
    pub mod scan_library;
}

pub mod library_scanners {
    pub mod factory;
}

pub mod queues {
    pub mod tokio;
}

pub mod persistence {
    pub mod models {
        pub mod file;
        pub mod library;
        pub mod media;
    }
    
    pub mod repositories {
        pub mod library;
    }
}

mod schema;

pub mod state;
