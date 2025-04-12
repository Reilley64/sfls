use crate::models::{InsertableMedia, Media};
use crate::schema::media;
use diesel::prelude::*;
use diesel::sql_types::BigInt;
use diesel_async::{AsyncPgConnection, RunQueryDsl};
use tracing::debug;

#[derive(Debug, Default)]
pub struct MediaCriteria {
    pub types: Option<Vec<String>>,
    pub title: Option<String>,
    pub library_id: Option<i64>,
    pub parent_id: Option<i64>,
}

pub async fn find_by_id(connection: &mut AsyncPgConnection, id: i64) -> QueryResult<Option<Media>> {
    media::dsl::media
        .find(id)
        .select(Media::as_select())
        .first(connection)
        .await
        .optional()
}

pub async fn find_by_season_and_parent_id(
    connection: &mut AsyncPgConnection,
    season: i32,
    parent_id: i64,
) -> QueryResult<Option<Media>> {
    media::dsl::media
        .filter(media::season.eq(season).and(media::parent_id.eq(parent_id)))
        .select(Media::as_select())
        .first(connection)
        .await
        .optional()
}

pub async fn find_by_episode_and_parent_id(
    connection: &mut AsyncPgConnection,
    episode: i32,
    parent_id: i64,
) -> QueryResult<Option<Media>> {
    media::dsl::media
        .filter(
            media::episode
                .eq(episode)
                .and(media::parent_id.eq(parent_id)),
        )
        .select(Media::as_select())
        .first(connection)
        .await
        .optional()
}

pub async fn find_by_path_and_parent_id(
    connection: &mut AsyncPgConnection,
    path: Option<String>,
    parent_id: Option<i64>,
) -> QueryResult<Option<Media>> {
    media::dsl::media
        .filter(media::path.eq(path).and(media::parent_id.eq(parent_id)))
        .select(Media::as_select())
        .first(connection)
        .await
        .optional()
}

pub async fn find_all(
    connection: &mut AsyncPgConnection,
    criteria: MediaCriteria,
) -> QueryResult<Vec<Media>> {
    let mut query = media::dsl::media.into_boxed();

    if let Some(types) = criteria.types {
        if !types.is_empty() {
            query = query.filter(media::dsl::type_.eq_any(types));
        }
    }

    if let Some(title) = criteria.title {
        query = query.filter(media::dsl::title.like(format!("%{title}%")));
    }

    if let Some(library_id) = criteria.library_id {
        query = query.filter(media::dsl::library_id.eq(library_id));
    }

    match criteria.parent_id {
        Some(parent_id) => {
            query = query.filter(media::dsl::parent_id.eq(parent_id));
        }
        None => {
            query = query.filter(media::dsl::parent_id.is_null());
        }
    }

    query = query.order(media::title);

    query.select(Media::as_select()).load(connection).await
}

pub async fn find_continue_watching(
    connection: &mut AsyncPgConnection,
    user_id: i64,
) -> QueryResult<Vec<Media>> {
    diesel::sql_query("
        WITH partially_watched_episodes AS (
            SELECT m.*
            FROM history h
            INNER JOIN media m ON h.media_id = m.id
            WHERE h.user_id = $1
            AND h.position < m.video_file_size * 0.9
        ),
        watched_episodes AS (
            SELECT DISTINCT ON(m.parent_id) m.parent_id, m.season, m.episode
            FROM history h
            INNER JOIN media m ON h.media_id = m.id
            WHERE h.user_id = $1
            AND m.type = 'tvshow'
            AND h.position >= m.video_file_size * 0.9
            ORDER BY m.parent_id, m.created_at DESC
        ),
        next_episodes AS (
            SELECT DISTINCT ON(m.parent_id) m.*
            FROM media m
            INNER JOIN watched_episodes w ON m.parent_id = w.parent_id AND m.episode IS NOT NULL AND (m.season > w.season OR (m.season = w.season AND m.episode > w.episode))
            ORDER BY m.parent_id, m.season, m.episode
        )
        SELECT m.id,
               m.created_at,
               m.updated_at,
               m.type AS type_,
               m.path,
               m.video_file,
               m.video_file_size,
               m.poster_file,
               m.thumbnail_file,
               m.fanart_file,
               m.logo_file,
               m.banner_file,
               m.title,
               m.season,
               m.episode,
               m.attributes,
               m.parent_id,
               m.library_id
        FROM (SELECT *
              FROM next_episodes
              UNION
              SELECT *
              FROM partially_watched_episodes) m
        ORDER BY created_at DESC;
    ")
        .bind::<BigInt, _>(user_id)
        .get_results::<Media>(connection)
        .await
}

pub async fn create(
    connection: &mut AsyncPgConnection,
    entity: &InsertableMedia,
) -> QueryResult<Media> {
    debug!("Creating media entity {:?}", entity);

    diesel::insert_into(media::table)
        .values(entity)
        .returning(Media::as_returning())
        .get_result(connection)
        .await
}

pub async fn update(connection: &mut AsyncPgConnection, entity: &Media) -> QueryResult<Media> {
    diesel::update(media::table)
        .filter(media::dsl::id.eq(entity.id))
        .set(entity)
        .returning(Media::as_returning())
        .get_result(connection)
        .await
}
