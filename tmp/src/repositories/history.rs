use crate::models::{History, InsertableHistory};
use crate::schema::history;
use diesel::prelude::*;
use diesel_async::{AsyncPgConnection, RunQueryDsl};

pub async fn create(
    connection: &mut AsyncPgConnection,
    entity: &InsertableHistory,
) -> QueryResult<History> {
    diesel::insert_into(history::table)
        .values(entity)
        .returning(History::as_returning())
        .get_result(connection)
        .await
}
