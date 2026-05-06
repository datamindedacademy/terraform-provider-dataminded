use axum::extract::Path;
use axum::extract::State;
use axum::http::StatusCode;
use axum::Json;
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool};
use diesel::SqliteConnection;
use utoipa_axum::router::OpenApiRouter;
use utoipa_axum::routes;

use super::error::internal_error;
use crate::model::user::{NewUser, User};
use crate::schema::users;

type AppPool = Pool<ConnectionManager<SqliteConnection>>;

pub fn user_routes() -> OpenApiRouter<AppPool> {
    OpenApiRouter::new()
        .routes(routes!(list_users, create_user))
        .routes(routes!(get_one_user, update_user, delete_user))
}

#[utoipa::path(
    get,
    path = "/",
    tag = "users",
    operation_id = "listUsers",
    description = "List users",
    responses((status = 200, description = "A list of users", body = [User]))
)]
async fn list_users(
    State(pool): State<AppPool>,
) -> Result<Json<Vec<User>>, (StatusCode, String)> {
    tracing::info!("Listing users");
    let mut conn = pool.get().map_err(internal_error)?;
    let res: Vec<User> = conn
        .transaction(|conn| users::table.select(User::as_select()).load(conn))
        .map_err(internal_error)?;
    Ok(Json(res))
}

#[utoipa::path(
    post,
    path = "/",
    tag = "users",
    operation_id = "createUser",
    description = "Create a new user",
    request_body = NewUser,
    responses((status = 201, description = "The created user", body = User))
)]
async fn create_user(
    State(pool): State<AppPool>,
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

#[utoipa::path(
    get,
    path = "/{id}",
    tag = "users",
    operation_id = "getUser",
    description = "Get a user by ID",
    params(("id" = i32, Path, description = "User ID")),
    responses((status = 200, description = "The requested user", body = User))
)]
async fn get_one_user(
    State(pool): State<AppPool>,
    Path(id): Path<i32>,
) -> Result<Json<User>, (StatusCode, String)> {
    tracing::info!("Getting user {:?}", id);
    let mut conn = pool.get().map_err(internal_error)?;
    let res = conn
        .transaction(|conn| users::table.find(id).first(conn))
        .map_err(internal_error)?;
    Ok(Json(res))
}

#[utoipa::path(
    put,
    path = "/{id}",
    tag = "users",
    operation_id = "updateUser",
    description = "Update a user by ID",
    params(("id" = i32, Path, description = "User ID")),
    request_body = NewUser,
    responses((status = 200, description = "The updated user", body = User))
)]
async fn update_user(
    State(pool): State<AppPool>,
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

#[utoipa::path(
    delete,
    path = "/{id}",
    tag = "users",
    operation_id = "deleteUser",
    description = "Delete a user by ID",
    params(("id" = i32, Path, description = "User ID")),
    responses((status = 202, description = "The deleted user", body = User))
)]
async fn delete_user(
    State(pool): State<AppPool>,
    Path(id): Path<i32>,
) -> Result<Json<User>, (StatusCode, String)> {
    tracing::info!("Deleting user: {:?}", id);
    let mut conn = pool.get().map_err(internal_error)?;
    let res = conn
        .transaction(|conn| diesel::delete(users::table.filter(users::id.eq(id))).get_result(conn))
        .map_err(internal_error)?;
    Ok(Json(res))
}
