use aide::axum::routing::get;
use aide::axum::ApiRouter;
use axum::extract::Path;
use axum::extract::State;
use axum::http::StatusCode;
use axum::Json;
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool};
use diesel::SqliteConnection;

use super::error::internal_error;
use crate::model::user::{NewUser, User};
use crate::schema::users;

pub fn user_routes() -> ApiRouter<Pool<ConnectionManager<SqliteConnection>>> {
    ApiRouter::new()
        .api_route("/", get(list_users).post(create_user))
        .api_route(
            "/:id",
            get(get_one_user).put(update_user).delete(delete_user),
        )
}

async fn create_user(
    State(pool): State<diesel::r2d2::Pool<diesel::r2d2::ConnectionManager<SqliteConnection>>>,
    Json(new_user): Json<NewUser>,
) -> Result<Json<User>, (StatusCode, String)> {
    tracing::info!("Creating user: {:?}", new_user);
    let mut conn = pool.get().map_err(internal_error)?;
    let res = conn
        .transaction(|conn| {
            diesel::insert_into(users::table)
                .values(new_user)
                .get_result(conn)
        })
        .map_err(internal_error)?;
    Ok(Json(res))
}

async fn list_users(
    State(pool): State<diesel::r2d2::Pool<diesel::r2d2::ConnectionManager<SqliteConnection>>>,
) -> Result<Json<Vec<User>>, (StatusCode, String)> {
    tracing::info!("Listing users");
    let mut conn = pool.get().map_err(internal_error)?;
    let res: Vec<User> = conn
        .transaction(|conn| users::table.select(User::as_select()).load(conn))
        .map_err(internal_error)?;
    Ok(Json(res))
}

async fn delete_user(
    State(pool): State<diesel::r2d2::Pool<diesel::r2d2::ConnectionManager<SqliteConnection>>>,
    Path(id): Path<i32>,
) -> Result<Json<User>, (StatusCode, String)> {
    tracing::info!("Deleting user: {:?}", id);
    let mut conn = pool.get().map_err(internal_error)?;
    let res = conn
        .transaction(|conn| diesel::delete(users::table.filter(users::id.eq(id))).get_result(conn))
        .map_err(internal_error)?;
    Ok(Json(res))
}

async fn update_user(
    State(pool): State<diesel::r2d2::Pool<diesel::r2d2::ConnectionManager<SqliteConnection>>>,
    Path(id): Path<i32>,
    Json(new_user): Json<NewUser>,
) -> Result<Json<User>, (StatusCode, String)> {
    tracing::info!("Updating user {:?} to {:?}", id, new_user);
    let mut conn = pool.get().map_err(internal_error)?;
    let res = conn
        .transaction(|conn| {
            diesel::update(users::table.find(id))
                .set(new_user)
                .get_result(conn)
        })
        .map_err(internal_error)?;
    Ok(Json(res))
}

async fn get_one_user(
    State(pool): State<diesel::r2d2::Pool<diesel::r2d2::ConnectionManager<SqliteConnection>>>,
    Path(id): Path<i32>,
) -> Result<Json<User>, (StatusCode, String)> {
    tracing::info!("Getting user {:?}", id);
    let mut conn = pool.get().map_err(internal_error)?;
    let res = conn
        .transaction(|conn| users::table.find(id).first(conn))
        .map_err(internal_error)?;
    Ok(Json(res))
}
