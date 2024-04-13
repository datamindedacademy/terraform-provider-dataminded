use axum::extract::Path;
use axum::extract::State;
use axum::http::StatusCode;
use axum::routing::{get, post};
use axum::{Json, Router};
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool};
use diesel::SqliteConnection;

use super::error::internal_error;
use crate::model::user::{NewUser, User};
use crate::schema::users;

pub fn user_routes() -> Router<Pool<ConnectionManager<SqliteConnection>>> {
    Router::new()
        .route("/list", get(list_users))
        .route("/create", post(create_user))
        .route(
            "/:id",
            get(get_one_user).put(update_user).delete(delete_user),
        )
}

async fn create_user(
    State(pool): State<diesel::r2d2::Pool<diesel::r2d2::ConnectionManager<SqliteConnection>>>,
    Json(new_user): Json<NewUser>,
) -> Result<Json<usize>, (StatusCode, String)> {
    let mut conn = pool.get().map_err(internal_error)?;

    tracing::info!("Creating user: {:?}", new_user);
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
    tracing::info!("Listing users");
    let res = conn
        .transaction(|conn| users::table.select(User::as_select()).load(conn))
        .map_err(internal_error)?;
    Ok(Json(res))
}

async fn delete_user(
    State(pool): State<diesel::r2d2::Pool<diesel::r2d2::ConnectionManager<SqliteConnection>>>,
    Path(id): Path<i32>,
) -> Result<Json<usize>, (StatusCode, String)> {
    let mut conn = pool.get().map_err(internal_error)?;
    tracing::info!("Deleting user: {:?}", id);
    let res = conn
        .transaction(|conn| diesel::delete(users::table.filter(users::id.eq(id))).execute(conn))
        .map_err(internal_error)?;
    Ok(Json(res))
}

async fn update_user(
    State(pool): State<diesel::r2d2::Pool<diesel::r2d2::ConnectionManager<SqliteConnection>>>,
    Path(id): Path<i32>,
    Json(new_user): Json<NewUser>,
) -> Result<Json<usize>, (StatusCode, String)> {
    tracing::info!("Updating user: {:?}", new_user);
    let mut conn = pool.get().map_err(internal_error)?;
    let res = conn
        .transaction(|conn| {
            diesel::update(users::table.find(id))
                .set(new_user)
                .execute(conn)
        })
        .map_err(internal_error)?;
    Ok(Json(res))
}

async fn get_one_user(
    State(pool): State<diesel::r2d2::Pool<diesel::r2d2::ConnectionManager<SqliteConnection>>>,
    Path(id): Path<i32>,
) -> Result<Json<User>, (StatusCode, String)> {
    tracing::info!("Getting user: {:?}", id);
    let mut conn = pool.get().map_err(internal_error)?;
    let res = conn
        .transaction(|conn| users::table.find(id).first(conn))
        .map_err(internal_error)?;
    Ok(Json(res))
}
