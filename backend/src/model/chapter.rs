use crate::schema::chapters;
use diesel::prelude::*;

#[derive(serde::Serialize, Selectable, Queryable)]
pub struct Chapter {
    id: i32,
    name: String,
}

#[derive(serde::Deserialize, Insertable)]
#[diesel(table_name = chapters)]
pub struct NewChapter {
    name: String,
}
