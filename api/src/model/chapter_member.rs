use crate::model::chapter::Chapter;
use crate::model::user::User;
use crate::schema::chapter_members;
use diesel::deserialize;
use diesel::expression::AsExpression;
use diesel::prelude::*;
use diesel::serialize;
use diesel::sql_types::*;
use diesel::sqlite::Sqlite;
use diesel::sqlite::SqliteValue;
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
    pub role: Option<ChapterRole>,
}

#[derive(serde::Deserialize, AsChangeset, JsonSchema, Debug)]
#[diesel(table_name = chapter_members)]
pub struct NewChapterMember {
    pub role: Option<ChapterRole>,
}

#[derive(serde::Serialize, Deserialize, JsonSchema, Debug, AsExpression)]
#[diesel(sql_type = diesel::sql_types::Text)]
pub enum ChapterRole {
    Contributor,
    Lead,
}

impl serialize::ToSql<Text, Sqlite> for ChapterRole {
    fn to_sql(&self, out: &mut serialize::Output<Sqlite>) -> serialize::Result {
        match self {
            ChapterRole::Lead => <str as serialize::ToSql<Text, Sqlite>>::to_sql("Lead", out),
            ChapterRole::Contributor => {
                <str as serialize::ToSql<Text, Sqlite>>::to_sql("Contributor", out)
            }
        }
    }
}

impl deserialize::FromSql<Text, Sqlite> for ChapterRole {
    fn from_sql(bytes: SqliteValue) -> deserialize::Result<Self> {
        let value = <String as deserialize::FromSql<Text, Sqlite>>::from_sql(bytes)?;
        match &value as &str {
            "Lead" => Ok(ChapterRole::Lead),
            "Contributor" => Ok(ChapterRole::Contributor),
            _ => Err("Unrecognized enum variant".into()),
        }
    }
}
