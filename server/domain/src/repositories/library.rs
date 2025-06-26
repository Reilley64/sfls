use crate::entities::library::Library;
use async_trait::async_trait;

#[async_trait]
pub trait LibraryRepository {
    async fn find_by_id(&self, id: &i64) -> anyhow::Result<Option<Library>>;
    
    async fn create(&self, library: &Library) -> anyhow::Result<Library>;
}
