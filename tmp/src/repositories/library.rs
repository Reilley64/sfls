use crate::models::{InsertableLibrary, Library};
use crate::schema::libraries;
use diesel::prelude::*;
use diesel_async::{AsyncPgConnection, RunQueryDsl};

pub async fn find_by_id(
    connection: &mut AsyncPgConnection,
    id: i64,
) -> QueryResult<Option<Library>> {
    libraries::dsl::libraries
        .find(id)
        .select(Library::as_select())
        .first(connection)
        .await
        .optional()
}

pub async fn create(
    connection: &mut AsyncPgConnection,
    entity: &InsertableLibrary,
) -> QueryResult<Library> {
    diesel::insert_into(libraries::table)
        .values(entity)
        .returning(Library::as_returning())
        .get_result(connection)
        .await
}
