use crate::schema::users;
use diesel::prelude::*;
use serde::Deserialize;

#[derive(serde::Serialize, Deserialize, Selectable, Queryable, AsChangeset, Debug)]
pub struct User {
    pub id: i32,
    pub name: String,
}

#[derive(serde::Deserialize, Insertable, AsChangeset, Debug)]
#[diesel(table_name = users)]
pub struct NewUser {
    pub name: String,
}
