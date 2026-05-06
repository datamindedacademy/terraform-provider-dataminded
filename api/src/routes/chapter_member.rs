use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::Json;
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool};
use diesel::SqliteConnection;
use utoipa_axum::router::OpenApiRouter;
use utoipa_axum::routes;

use super::error::internal_error;
use crate::model::chapter_member::{ChapterMember, ChapterRole, NewChapterMember};
use crate::schema::chapter_members;

type AppPool = Pool<ConnectionManager<SqliteConnection>>;

pub fn chapter_member_routes() -> OpenApiRouter<AppPool> {
    OpenApiRouter::new()
        .routes(routes!(list_chapter_members))
        .routes(routes!(
            get_one_chapter_member,
            create_chapter_member,
            update_chapter_member,
            delete_chapter_member
        ))
}

#[utoipa::path(
    get,
    path = "/",
    tag = "chapter_members",
    operation_id = "listChapterMembersInChapter",
    description = "List chapter members for a chapter",
    params(("id" = i32, Path, description = "Chapter ID")),
    responses((status = 200, description = "A list of chapter members", body = [ChapterMember]))
)]
async fn list_chapter_members(
    State(pool): State<AppPool>,
    Path(chapter_id): Path<i32>,
) -> Result<Json<Vec<ChapterMember>>, (StatusCode, String)> {
    tracing::info!("Listing chapter members for chapter {:?}", chapter_id);
    load_members(pool, Some(chapter_id))
}

pub fn load_all_members(pool: AppPool) -> Result<Vec<ChapterMember>, (StatusCode, String)> {
    let mut conn = pool.get().map_err(internal_error)?;
    conn.transaction(|conn| {
        chapter_members::table
            .select(ChapterMember::as_select())
            .load(conn)
    })
    .map_err(internal_error)
}

fn load_members(
    pool: AppPool,
    chapter_id: Option<i32>,
) -> Result<Json<Vec<ChapterMember>>, (StatusCode, String)> {
    let mut conn = pool.get().map_err(internal_error)?;
    let res = conn
        .transaction(|conn| {
            match chapter_id {
                Some(cid) => chapter_members::table
                    .into_boxed()
                    .filter(chapter_members::chapter_id.eq(cid)),
                None => chapter_members::table.into_boxed(),
            }
            .select(ChapterMember::as_select())
            .load(conn)
        })
        .map_err(internal_error)?;
    Ok(Json(res))
}

#[utoipa::path(
    get,
    path = "/{user_id}",
    tag = "chapter_members",
    operation_id = "getChapterMember",
    description = "Get a chapter member by ID",
    params(
        ("id" = i32, Path, description = "Chapter ID"),
        ("user_id" = i32, Path, description = "User ID")
    ),
    responses((status = 200, description = "The requested chapter member", body = ChapterMember))
)]
async fn get_one_chapter_member(
    State(pool): State<AppPool>,
    Path((chapter_id, user_id)): Path<(i32, i32)>,
) -> Result<Json<ChapterMember>, (StatusCode, String)> {
    tracing::info!(
        "Getting user {:?} as member of chapter {:?}",
        user_id,
        chapter_id
    );
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

#[utoipa::path(
    post,
    path = "/{user_id}",
    tag = "chapter_members",
    operation_id = "createChapterMember",
    description = "Create a new chapter member",
    params(
        ("id" = i32, Path, description = "Chapter ID"),
        ("user_id" = i32, Path, description = "User ID")
    ),
    request_body = NewChapterMember,
    responses((status = 201, description = "The created chapter member", body = ChapterMember))
)]
async fn create_chapter_member(
    State(pool): State<AppPool>,
    Path((chapter_id, user_id)): Path<(i32, i32)>,
    Json(new_chapter_member): Json<NewChapterMember>,
) -> Result<Json<ChapterMember>, (StatusCode, String)> {
    tracing::info!(
        "Registering user {:?} as member in chapter {:?} with role {:?}",
        user_id,
        chapter_id,
        new_chapter_member.role
    );
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

#[utoipa::path(
    put,
    path = "/{user_id}",
    tag = "chapter_members",
    operation_id = "updateChapterMember",
    description = "Update a chapter member by ID",
    params(
        ("id" = i32, Path, description = "Chapter ID"),
        ("user_id" = i32, Path, description = "User ID")
    ),
    request_body = NewChapterMember,
    responses((status = 200, description = "The updated chapter member", body = ChapterMember))
)]
async fn update_chapter_member(
    State(pool): State<AppPool>,
    Path((chapter_id, user_id)): Path<(i32, i32)>,
    Json(chapter_member): Json<NewChapterMember>,
) -> Result<Json<ChapterMember>, (StatusCode, String)> {
    tracing::info!(
        "Changing member {:?} of chapter {:?} to role {:?}",
        user_id,
        chapter_id,
        chapter_member.role
    );

    let mut chapter_member = chapter_member;
    if chapter_member.role.is_none() {
        chapter_member.role = Some(ChapterRole::Contributor);
    }
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

#[utoipa::path(
    delete,
    path = "/{user_id}",
    tag = "chapter_members",
    operation_id = "deleteChapterMember",
    description = "Delete a chapter member by ID",
    params(
        ("id" = i32, Path, description = "Chapter ID"),
        ("user_id" = i32, Path, description = "User ID")
    ),
    responses((status = 202, description = "The deleted chapter member", body = ChapterMember))
)]
async fn delete_chapter_member(
    State(pool): State<AppPool>,
    Path((chapter_id, user_id)): Path<(i32, i32)>,
) -> Result<Json<ChapterMember>, (StatusCode, String)> {
    tracing::info!(
        "Deleting user {:?} as member of chapter {:?}",
        user_id,
        chapter_id
    );
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
