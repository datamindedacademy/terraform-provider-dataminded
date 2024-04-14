use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::routing::get;
use axum::{Json, Router};
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool};
use diesel::SqliteConnection;

use super::error::internal_error;
use crate::model::chapter_member::{ChapterMember, NewChapterMember};
use crate::schema::chapter_members;

pub fn chapter_member_routes() -> Router<Pool<ConnectionManager<SqliteConnection>>> {
    Router::new().route("/", get(list_chapter_members)).route(
        "/:user_id",
        get(get_one_chapter_member)
            .post(create_chapter_member)
            .put(update_chapter_member)
            .delete(delete_chapter_member),
    )
}

async fn create_chapter_member(
    State(pool): State<diesel::r2d2::Pool<diesel::r2d2::ConnectionManager<SqliteConnection>>>,
    Path((chapter_id, user_id)): Path<(i32, i32)>,
    Json(new_chapter_member): Json<NewChapterMember>,
) -> Result<Json<ChapterMember>, (StatusCode, String)> {
    tracing::info!("Creating chapter member: {:?}", new_chapter_member);
    let mut conn = pool.get().map_err(internal_error)?;
    let res = conn
        .transaction(|conn| {
            diesel::insert_into(chapter_members::table)
                .values(ChapterMember {
                    chapter_id,
                    user_id,
                    role: new_chapter_member.role,
                })
                .get_result(conn)
        })
        .map_err(internal_error)?;
    Ok(Json(res))
}

pub async fn list_chapter_members(
    State(pool): State<diesel::r2d2::Pool<diesel::r2d2::ConnectionManager<SqliteConnection>>>,
    optional_path: Option<Path<i32>>,
) -> Result<Json<Vec<ChapterMember>>, (StatusCode, String)> {
    tracing::info!("Listing chapter members");
    let mut conn = pool.get().map_err(internal_error)?;
    let res = conn
        .transaction(|conn| {
            match optional_path {
                Some(Path(chapter_id)) => chapter_members::table
                    .into_boxed()
                    .filter(chapter_members::chapter_id.eq(chapter_id)),
                None => chapter_members::table.into_boxed(),
            }
            .select(ChapterMember::as_select())
            .load(conn)
        })
        .map_err(internal_error)?;
    Ok(Json(res))
}

async fn update_chapter_member(
    State(pool): State<diesel::r2d2::Pool<diesel::r2d2::ConnectionManager<SqliteConnection>>>,
    Path((chapter_id, user_id)): Path<(i32, i32)>,
    Json(chapter_member): Json<NewChapterMember>,
) -> Result<Json<ChapterMember>, (StatusCode, String)> {
    tracing::info!("Updating chapter member: {:?}", (chapter_id, user_id));
    let mut conn = pool.get().map_err(internal_error)?;
    let res = conn
        .transaction(|conn| {
            diesel::update(
                chapter_members::table.filter(
                    chapter_members::chapter_id
                        .eq(chapter_id)
                        .and(chapter_members::user_id.eq(user_id)),
                ),
            )
            .set(chapter_member)
            .get_result(conn)
        })
        .map_err(internal_error)?;
    Ok(Json(res))
}

async fn delete_chapter_member(
    State(pool): State<diesel::r2d2::Pool<diesel::r2d2::ConnectionManager<SqliteConnection>>>,
    Path((chapter_id, user_id)): Path<(i32, i32)>,
) -> Result<Json<ChapterMember>, (StatusCode, String)> {
    tracing::info!("Deleting chapter member: {:?}", (chapter_id, user_id));
    let mut conn = pool.get().map_err(internal_error)?;
    let res = conn
        .transaction(|conn| {
            diesel::delete(
                chapter_members::table.filter(
                    chapter_members::chapter_id
                        .eq(chapter_id)
                        .and(chapter_members::user_id.eq(user_id)),
                ),
            )
            .get_result(conn)
        })
        .map_err(internal_error)?;
    Ok(Json(res))
}

async fn get_one_chapter_member(
    State(pool): State<diesel::r2d2::Pool<diesel::r2d2::ConnectionManager<SqliteConnection>>>,
    Path((chapter_id, user_id)): Path<(i32, i32)>,
) -> Result<Json<ChapterMember>, (StatusCode, String)> {
    tracing::info!("Getting chapter member: {:?}", (chapter_id, user_id));
    let mut conn = pool.get().map_err(internal_error)?;
    let res = conn
        .transaction(|conn| {
            chapter_members::table
                .filter(
                    chapter_members::chapter_id
                        .eq(chapter_id)
                        .and(chapter_members::user_id.eq(user_id)),
                )
                .select(ChapterMember::as_select())
                .first(conn)
        })
        .map_err(internal_error)?;
    Ok(Json(res))
}
