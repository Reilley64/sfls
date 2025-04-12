use crate::models::Library;
use crate::schema::libraries;
use async_trait::async_trait;
use diesel::prelude::*;
use snowflake::SnowflakeIdGenerator;
use std::sync::Arc;
use tokio::sync::Mutex;

#[async_trait]
pub trait LibraryRepositoryTrait {
    async fn find_by_id(&self, connection: &mut SqliteConnection, id: i64) -> QueryResult<Option<Library>>;

    async fn create(&self, connection: &mut SqliteConnection, entity: &mut Library) -> QueryResult<Library>;
}

pub struct LibraryRepository {
    pub id_generator: Arc<Mutex<SnowflakeIdGenerator>>,
}

impl LibraryRepository {
    pub fn new(id_generator: Arc<Mutex<SnowflakeIdGenerator>>) -> Self {
        Self {
            id_generator,
        }
    }
}

#[async_trait]
impl LibraryRepositoryTrait for LibraryRepository {
    async fn find_by_id(&self, connection: &mut SqliteConnection, id: i64) -> QueryResult<Option<Library>> {
        libraries::dsl::libraries.find(id).select(Library::as_select()).first(connection).optional()
    }

    async fn create(&self, connection: &mut SqliteConnection, entity: &mut Library) -> QueryResult<Library> {
        {
            let mut id_generator = self.id_generator.lock().await;
            entity.id = id_generator.real_time_generate();
        }

        let now = chrono::Utc::now();
        entity.created_at = now.to_string();
        entity.updated_at = now.to_string();

        diesel::insert_into(libraries::table)
            .values(&*entity)
            .returning(Library::as_returning())
            .get_result(connection)
    }
}