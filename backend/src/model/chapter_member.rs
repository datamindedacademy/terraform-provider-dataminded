use crate::schema::chapter_members;
use diesel::prelude::*;

#[derive(serde::Serialize, Selectable, Queryable)]
struct ChapterMember {
    pub id: i32,
    pub chapter_id: i32,
    pub user_id: i32,
}

#[derive(serde::Deserialize, Insertable)]
#[diesel(table_name = chapter_members)]
struct NewChapterMember {
    pub chapter_id: i32,
    pub user_id: i32,
}
