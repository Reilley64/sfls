use crate::models::Media;
use crate::schema::media;
use diesel::prelude::*;
use snowflake::SnowflakeIdGenerator;
use std::sync::Arc;
use async_trait::async_trait;
use tokio::sync::Mutex;
use tracing::debug;

pub struct MediaCriteria {
    pub type_: Vec<String>,
    pub title: Option<String>,
    pub parent_id: Option<i64>,
}

#[async_trait]
pub trait MediaRepositoryTrait {
    async fn find_by_id(&self, connection: &mut SqliteConnection, id: i64) -> QueryResult<Option<Media>>;

    async fn find_all(&self, connection: &mut SqliteConnection, criteria: MediaCriteria) -> QueryResult<Vec<Media>>;

    async fn create(&self, connection: &mut SqliteConnection, entity: &mut Media) -> QueryResult<Media>;
}

pub struct MediaRepository {
    pub id_generator: Arc<Mutex<SnowflakeIdGenerator>>,
}

impl MediaRepository {
    pub fn new(id_generator: Arc<Mutex<SnowflakeIdGenerator>>) -> Self {
        Self {
            id_generator
        }
    }
}

#[async_trait]
impl MediaRepositoryTrait for MediaRepository {
    async fn find_by_id(&self, connection: &mut SqliteConnection, id: i64) -> QueryResult<Option<Media>> {
        media::dsl::media.find(id).select(Media::as_select()).first(connection).optional()
    }

    async fn find_all(&self, connection: &mut SqliteConnection, criteria: MediaCriteria) -> QueryResult<Vec<Media>> {
        let mut query = media::dsl::media.into_boxed();

        if !criteria.type_.is_empty() {
            query = query.filter(media::dsl::type_.eq_any(criteria.type_));
        }

        if let Some(title) = criteria.title {
            query = query.filter(media::dsl::title.like(format!("%{}%", title)));
        }

        if let Some(parent_id) = criteria.parent_id {
            query = query.filter(media::dsl::parent_id.eq(parent_id));
        }

        query.select(Media::as_select()).load(connection)
    }

    async fn create(&self, connection: &mut SqliteConnection, entity: &mut Media) -> QueryResult<Media> {
        debug!("Creating media entity {:?}", entity);

        {
            let mut id_generator = self.id_generator.lock().await;
            entity.id = id_generator.real_time_generate();
        }

        let now = chrono::Utc::now();
        entity.created_at = now.to_string();
        entity.updated_at = now.to_string();

        diesel::insert_into(media::table)
            .values(&*entity)
            .returning(Media::as_returning())
            .get_result(connection)
    }
}