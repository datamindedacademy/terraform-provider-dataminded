use crate::schema::chapter_members;
use diesel::prelude::*;

#[derive(serde::Serialize, Selectable, Queryable)]
struct ChapterMember {
    id: i32,
    chapter_id: i32,
    user_id: i32,
}

#[derive(serde::Deserialize, Insertable)]
#[diesel(table_name = chapter_members)]
struct NewChapterMember {
    chapter_id: i32,
    user_id: i32,
}
