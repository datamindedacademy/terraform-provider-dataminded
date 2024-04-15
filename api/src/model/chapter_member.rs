use crate::model::chapter::Chapter;
use crate::model::user::User;
use crate::schema::chapter_members;
use diesel::prelude::*;
use schemars::JsonSchema;
use serde::Deserialize;

#[derive(
    serde::Serialize,
    Deserialize,
    Selectable,
    Associations,
    Insertable,
    AsChangeset,
    Queryable,
    JsonSchema,
    Debug,
)]
#[diesel(belongs_to(Chapter))]
#[diesel(belongs_to(User))]
#[diesel(primary_key(chapter_id, user_id))]
pub struct ChapterMember {
    pub chapter_id: i32,
    pub user_id: i32,
    pub role: Option<String>,
}

#[derive(serde::Deserialize, AsChangeset, JsonSchema, Debug)]
#[diesel(table_name = chapter_members)]
pub struct NewChapterMember {
    pub role: Option<String>,
}
