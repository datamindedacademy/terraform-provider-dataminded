use crate::schema::chapters;
use diesel::prelude::*;

#[derive(serde::Serialize, Selectable, Queryable)]
pub struct Chapter {
    pub id: i32,
    pub name: String,
}

#[derive(serde::Deserialize, Insertable, AsChangeset, Debug)]
#[diesel(table_name = chapters)]
pub struct NewChapter {
    pub name: String,
}
