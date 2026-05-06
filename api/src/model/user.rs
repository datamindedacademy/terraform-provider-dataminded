use crate::schema::users;
use diesel::prelude::*;
use serde::Deserialize;
use utoipa::ToSchema;

#[derive(
    serde::Serialize,
    Deserialize,
    Selectable,
    Queryable,
    AsChangeset,
    ToSchema,
    Debug,
)]
pub struct User {
    pub id: i32,
    pub name: String,
}

#[derive(serde::Deserialize, Insertable, AsChangeset, Queryable, ToSchema, Debug)]
#[diesel(table_name = users)]
pub struct NewUser {
    pub name: String,
}
