use axum::extract::Path;
use axum::extract::State;
use axum::http::StatusCode;
use axum::routing::{get, post};
use axum::{Json, Router};
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool};
use diesel::SqliteConnection;

use super::error::internal_error;
use crate::model::chapter::{Chapter, NewChapter};
use crate::schema::chapters;

pub fn chapter_routes() -> Router<Pool<ConnectionManager<SqliteConnection>>> {
    Router::new()
        .route("/list", get(list_chapters))
        .route("/create", post(create_chapter))
        .route(
            "/:id",
            get(get_one_chapter)
                .put(update_chapter)
                .delete(delete_chapter),
        )
}

async fn create_chapter(
    State(pool): State<diesel::r2d2::Pool<diesel::r2d2::ConnectionManager<SqliteConnection>>>,
    Json(new_chapter): Json<NewChapter>,
) -> Result<Json<usize>, (StatusCode, String)> {
    let mut conn = pool.get().map_err(internal_error)?;
    let res = conn
        .transaction(|conn| {
            diesel::insert_into(chapters::table)
                .values(new_chapter)
                .execute(conn)
        })
        .map_err(internal_error)?;
    Ok(Json(res))
}

async fn list_chapters(
    State(pool): State<diesel::r2d2::Pool<diesel::r2d2::ConnectionManager<SqliteConnection>>>,
) -> Result<Json<Vec<Chapter>>, (StatusCode, String)> {
    let mut conn = pool.get().map_err(internal_error)?;
    let res = conn
        .transaction(|conn| chapters::table.select(Chapter::as_select()).load(conn))
        .map_err(internal_error)?;
    Ok(Json(res))
}

async fn delete_chapter(
    State(pool): State<diesel::r2d2::Pool<diesel::r2d2::ConnectionManager<SqliteConnection>>>,
    Path(id): Path<i32>,
) -> Result<Json<usize>, (StatusCode, String)> {
    let mut conn = pool.get().map_err(internal_error)?;
    let res = conn
        .transaction(|conn| {
            diesel::delete(chapters::table.filter(chapters::id.eq(id))).execute(conn)
        })
        .map_err(internal_error)?;
    Ok(Json(res))
}

async fn update_chapter(
    State(pool): State<diesel::r2d2::Pool<diesel::r2d2::ConnectionManager<SqliteConnection>>>,
    Path(id): Path<i32>,
    Json(new_chapter): Json<NewChapter>,
) -> Result<Json<usize>, (StatusCode, String)> {
    let mut conn = pool.get().map_err(internal_error)?;
    let res = conn
        .transaction(|conn| {
            diesel::update(chapters::table.filter(chapters::id.eq(id)))
                .set(new_chapter)
                .execute(conn)
        })
        .map_err(internal_error)?;
    Ok(Json(res))
}

async fn get_one_chapter(
    State(pool): State<diesel::r2d2::Pool<diesel::r2d2::ConnectionManager<SqliteConnection>>>,
    Path(id): Path<i32>,
) -> Result<Json<Chapter>, (StatusCode, String)> {
    let mut conn = pool.get().map_err(internal_error)?;
    let res = conn
        .transaction(|conn| {
            chapters::table
                .filter(chapters::id.eq(id))
                .select(Chapter::as_select())
                .first(conn)
        })
        .map_err(internal_error)?;
    Ok(Json(res))
}
