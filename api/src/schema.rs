use diesel::prelude::*;

table! {
    users (id) {
        id -> Integer,
        name -> Text,
    }
}

table! {
    chapters (id) {
        id -> Integer,
        name -> Text
    }
}

table! {
    chapter_members (chapter_id, user_id) {
        chapter_id -> Integer,
        user_id -> Integer,
        role -> Nullable<Text>,
    }
}

diesel::joinable!(chapter_members -> chapters (chapter_id));
diesel::joinable!(chapter_members -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(users, chapters, chapter_members);
