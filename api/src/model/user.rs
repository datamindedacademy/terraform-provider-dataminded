use crate::schema::users;
use aide::OperationIo;
use diesel::prelude::*;
use schemars::JsonSchema;
use serde::Deserialize;

#[derive(
    serde::Serialize,
    Deserialize,
    Selectable,
    Queryable,
    AsChangeset,
    JsonSchema,
    OperationIo,
    Debug,
)]
pub struct User {
    pub id: i32,
    pub name: String,
}

#[derive(serde::Deserialize, Insertable, AsChangeset, Queryable, JsonSchema, Debug)]
#[diesel(table_name = users)]
pub struct NewUser {
    pub name: String,
}
