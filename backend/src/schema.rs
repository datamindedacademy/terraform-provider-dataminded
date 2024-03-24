use diesel::prelude::*;

table! {
    users (id) {
        id -> Integer,
        name -> Text,
    }
}
