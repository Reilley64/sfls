use crate::persistence::models::library::{InsertableLibrary, Library};
use crate::schema::libraries;
use crate::state::DbPool;
use async_trait::async_trait;
use diesel::prelude::*;
use diesel_async::RunQueryDsl;
use domain::repositories::library::LibraryRepository;

pub struct LibraryRepositoryImpl {
    pool: DbPool,
}

impl LibraryRepositoryImpl {
    pub fn new(pool: DbPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl LibraryRepository for LibraryRepositoryImpl {
    async fn find_by_id(
        &self,
        id: &i64,
    ) -> anyhow::Result<Option<domain::entities::library::Library>> {
        let connection = &mut self.pool.get().await?;
        let model = libraries::dsl::libraries
            .find(id)
            .select(Library::as_select())
            .first(connection)
            .await
            .optional()?;
        Ok(model.map(|m| m.into()))
    }

    async fn create(
        &self,
        library: &domain::entities::library::Library,
    ) -> anyhow::Result<domain::entities::library::Library> {
        let connection = &mut self.pool.get().await?;
        let model = diesel::insert_into(libraries::table)
            .values(InsertableLibrary {
                created_by: library.created_by.clone(),
                updated_by: library.updated_by.clone(),
                name: library.name.clone(),
                path: library.path.clone(),
                media_type: library.media_type.clone().into(),
            })
            .returning(Library::as_returning())
            .get_result(connection)
            .await?;
        Ok(model.into())
    }
}
