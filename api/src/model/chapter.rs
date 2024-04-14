use crate::schema::chapters;
use diesel::prelude::*;
use schemars::JsonSchema;

#[derive(serde::Serialize, Selectable, JsonSchema, Queryable)]
pub struct Chapter {
    pub id: i32,
    pub name: String,
}

#[derive(serde::Deserialize, Insertable, AsChangeset, JsonSchema, Debug)]
#[diesel(table_name = chapters)]
pub struct NewChapter {
    pub name: String,
}
