pub mod commands {
    pub mod command;
    
    pub mod library {
        pub mod create;
        pub mod scan;
        pub mod scan_folder;
    }
}

pub mod ports {
    pub mod job;
    pub mod job_factory;
    pub mod queue;
}
