use crate::models::{InsertableUser, User};
use crate::schema::users;
use diesel::prelude::*;
use diesel_async::{AsyncPgConnection, RunQueryDsl};
use tracing::debug;

pub async fn find_by_id(connection: &mut AsyncPgConnection, id: i64) -> QueryResult<Option<User>> {
    users::dsl::users
        .find(id)
        .select(User::as_select())
        .first(connection)
        .await
        .optional()
}

pub async fn find_by_email(
    connection: &mut AsyncPgConnection,
    email: String,
) -> QueryResult<Option<User>> {
    users::dsl::users
        .filter(users::email.eq(email))
        .select(User::as_select())
        .first(connection)
        .await
        .optional()
}

pub async fn count(connection: &mut AsyncPgConnection) -> QueryResult<i64> {
    users::dsl::users.count().get_result(connection).await
}

pub async fn create(
    connection: &mut AsyncPgConnection,
    entity: &InsertableUser,
) -> QueryResult<User> {
    debug!("Creating user entity {:?}", entity);

    diesel::insert_into(users::table)
        .values(entity)
        .returning(User::as_returning())
        .get_result(connection)
        .await
}
