use axum::extract::State;
use axum::http::StatusCode;
use axum::routing::{get, post};
use axum::{Json, Router};
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool};
use diesel::SqliteConnection;

use super::error::internal_error;
use crate::model::chapter_member::ChapterMember;
use crate::schema::chapter_members;

pub fn chapter_member_routes() -> Router<Pool<ConnectionManager<SqliteConnection>>> {
    Router::new()
        .route("/list", get(list_chapter_members))
        .route("/create", post(create_chapter_member))
        .route(
            "/:id",
            get(get_one_chapter_member)
                .put(update_chapter_member)
                .delete(delete_chapter_member),
        )
}

async fn create_chapter_member(
    State(pool): State<diesel::r2d2::Pool<diesel::r2d2::ConnectionManager<SqliteConnection>>>,
    Json(new_chapter_member): Json<ChapterMember>,
) -> Result<Json<usize>, (StatusCode, String)> {
    tracing::info!("Creating chapter member: {:?}", new_chapter_member);
    let mut conn = pool.get().map_err(internal_error)?;
    let res = conn
        .transaction(|conn| {
            diesel::insert_into(chapter_members::table)
                .values(new_chapter_member)
                .execute(conn)
        })
        .map_err(internal_error)?;
    Ok(Json(res))
}

async fn list_chapter_members(
    State(pool): State<diesel::r2d2::Pool<diesel::r2d2::ConnectionManager<SqliteConnection>>>,
) -> Result<Json<Vec<ChapterMember>>, (StatusCode, String)> {
    tracing::info!("Listing chapter members");
    let mut conn = pool.get().map_err(internal_error)?;
    let res = conn
        .transaction(|conn| {
            chapter_members::table
                .select(ChapterMember::as_select())
                .load(conn)
        })
        .map_err(internal_error)?;
    Ok(Json(res))
}

async fn update_chapter_member(
    State(pool): State<diesel::r2d2::Pool<diesel::r2d2::ConnectionManager<SqliteConnection>>>,
    Json(chapter_member): Json<ChapterMember>,
) -> Result<Json<usize>, (StatusCode, String)> {
    let mut conn = pool.get().map_err(internal_error)?;
    let res = conn
        .transaction(|conn| {
            diesel::update(
                chapter_members::table.filter(
                    chapter_members::chapter_id
                        .eq(chapter_member.chapter_id)
                        .and(chapter_members::user_id.eq(chapter_member.user_id)),
                ),
            )
            .set(chapter_member)
            .execute(conn)
        })
        .map_err(internal_error)?;
    Ok(Json(res))
}

async fn delete_chapter_member(
    State(pool): State<diesel::r2d2::Pool<diesel::r2d2::ConnectionManager<SqliteConnection>>>,
    Json(chapter_member): Json<ChapterMember>,
) -> Result<Json<usize>, (StatusCode, String)> {
    let mut conn = pool.get().map_err(internal_error)?;
    let res = conn
        .transaction(|conn| {
            diesel::delete(
                chapter_members::table.filter(
                    chapter_members::chapter_id
                        .eq(chapter_member.chapter_id)
                        .and(chapter_members::user_id.eq(chapter_member.user_id)),
                ),
            )
            .execute(conn)
        })
        .map_err(internal_error)?;
    Ok(Json(res))
}

async fn get_one_chapter_member(
    State(pool): State<diesel::r2d2::Pool<diesel::r2d2::ConnectionManager<SqliteConnection>>>,
    Json(chapter_member): Json<ChapterMember>,
) -> Result<Json<ChapterMember>, (StatusCode, String)> {
    let mut conn = pool.get().map_err(internal_error)?;
    let res = conn
        .transaction(|conn| {
            chapter_members::table
                .filter(
                    chapter_members::chapter_id
                        .eq(chapter_member.chapter_id)
                        .and(chapter_members::user_id.eq(chapter_member.user_id)),
                )
                .select(ChapterMember::as_select())
                .first(conn)
        })
        .map_err(internal_error)?;
    Ok(Json(res))
}
