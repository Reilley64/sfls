pub mod fetch_artwork;
pub mod scan_folder;
pub mod scan_library;
pub mod scan_season_folder;

use async_trait::async_trait;

#[async_trait]
pub trait Job: Send + Sync {
    async fn run(&self) -> Result<(), anyhow::Error>;
}
