use crate::model::chapter::Chapter;
use crate::model::user::User;
use crate::schema::chapter_members;
use diesel::prelude::*;
use serde::Deserialize;

#[derive(serde::Serialize, Deserialize, Selectable, Associations, Insertable, Queryable, Debug)]
#[diesel(belongs_to(Chapter))]
#[diesel(belongs_to(User))]
#[diesel(primary_key(chapter_id, user_id))]
pub struct ChapterMember {
    pub chapter_id: i32,
    pub user_id: i32,
}
