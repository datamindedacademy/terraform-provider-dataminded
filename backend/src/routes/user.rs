use axum::extract::State;
use axum::http::StatusCode;
use axum::routing::{get, post};
use axum::{Json, Router};
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool};
use diesel::SqliteConnection;

#[path = "../error.rs"]
mod error;
use crate::model::user::{NewUser, User};
use crate::schema::users;
use error::internal_error;

pub fn user_routes() -> Router<Pool<ConnectionManager<SqliteConnection>>> {
    Router::new()
        .route("/user/list", get(list_users))
        .route("/user/create", post(create_user))
    // .route("/user/update", post(update_user))
    // .route("/user/delete", post(delete_user))
}

async fn create_user(
    State(pool): State<diesel::r2d2::Pool<diesel::r2d2::ConnectionManager<SqliteConnection>>>,
    Json(new_user): Json<NewUser>,
) -> Result<Json<usize>, (StatusCode, String)> {
    let mut conn = pool.get().map_err(internal_error)?;
    let res = conn
        .transaction(|conn| {
            diesel::insert_into(users::table)
                .values(new_user)
                .execute(conn)
        })
        .map_err(internal_error)?;
    Ok(Json(res))
}

async fn list_users(
    State(pool): State<diesel::r2d2::Pool<diesel::r2d2::ConnectionManager<SqliteConnection>>>,
) -> Result<Json<Vec<User>>, (StatusCode, String)> {
    let mut conn = pool.get().map_err(internal_error)?;
    let res = conn
        .transaction(|conn| users::table.select(User::as_select()).load(conn))
        .map_err(internal_error)?;
    Ok(Json(res))
}
