use axum::extract::Path;
use axum::extract::State;
use axum::http::StatusCode;
use axum::Json;
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool};
use diesel::SqliteConnection;
use utoipa_axum::router::OpenApiRouter;
use utoipa_axum::routes;

use super::chapter_member::{chapter_member_routes, load_all_members};
use super::error::internal_error;
use crate::model::chapter::{Chapter, NewChapter};
use crate::model::chapter_member::ChapterMember;
use crate::schema::chapters;

type AppPool = Pool<ConnectionManager<SqliteConnection>>;

pub fn chapter_routes() -> OpenApiRouter<AppPool> {
    OpenApiRouter::new()
        .routes(routes!(list_chapters, create_chapter))
        .routes(routes!(get_one_chapter, update_chapter, delete_chapter))
        .routes(routes!(list_all_chapter_members))
        .nest("/{id}/member/", chapter_member_routes())
}

#[utoipa::path(
    get,
    path = "/member/",
    tag = "chapter_members",
    operation_id = "listChapterMembers",
    description = "List all chapter members",
    responses((status = 200, description = "A list of chapter members", body = [ChapterMember]))
)]
async fn list_all_chapter_members(
    State(pool): State<AppPool>,
) -> Result<Json<Vec<ChapterMember>>, (StatusCode, String)> {
    tracing::info!("Listing all chapter members");
    load_all_members(pool).map_err(|(s, m)| (s, m)).map(Json)
}

#[utoipa::path(
    get,
    path = "/",
    tag = "chapters",
    operation_id = "listChapters",
    description = "List chapters",
    responses((status = 200, description = "A list of chapters", body = [Chapter]))
)]
async fn list_chapters(
    State(pool): State<AppPool>,
) -> Result<Json<Vec<Chapter>>, (StatusCode, String)> {
    tracing::info!("Listing chapters");
    let mut conn = pool.get().map_err(internal_error)?;
    let res = conn
        .transaction(|conn| chapters::table.select(Chapter::as_select()).load(conn))
        .map_err(internal_error)?;
    Ok(Json(res))
}

#[utoipa::path(
    post,
    path = "/",
    tag = "chapters",
    operation_id = "createChapter",
    description = "Create a new chapter",
    request_body = NewChapter,
    responses((status = 201, description = "The created chapter", body = Chapter))
)]
async fn create_chapter(
    State(pool): State<AppPool>,
    Json(new_chapter): Json<NewChapter>,
) -> Result<Json<Chapter>, (StatusCode, String)> {
    tracing::info!("Creating chapter: {:?}", new_chapter);
    let mut conn = pool.get().map_err(internal_error)?;
    let res = conn
        .transaction(|conn| {
            diesel::insert_into(chapters::table)
                .values(new_chapter)
                .get_result(conn)
        })
        .map_err(internal_error)?;
    Ok(Json(res))
}

#[utoipa::path(
    get,
    path = "/{id}",
    tag = "chapters",
    operation_id = "getChapter",
    description = "Get a chapter by ID",
    params(("id" = i32, Path, description = "Chapter ID")),
    responses((status = 200, description = "The requested chapter", body = Chapter))
)]
async fn get_one_chapter(
    State(pool): State<AppPool>,
    Path(id): Path<i32>,
) -> Result<Json<Chapter>, (StatusCode, String)> {
    tracing::info!("Getting chapter {:?}", id);
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

#[utoipa::path(
    put,
    path = "/{id}",
    tag = "chapters",
    operation_id = "updateChapter",
    description = "Update a chapter by ID",
    params(("id" = i32, Path, description = "Chapter ID")),
    request_body = NewChapter,
    responses((status = 200, description = "The updated chapter", body = Chapter))
)]
async fn update_chapter(
    State(pool): State<AppPool>,
    Path(id): Path<i32>,
    Json(new_chapter): Json<NewChapter>,
) -> Result<Json<Chapter>, (StatusCode, String)> {
    tracing::info!("Updating chapter {:?} to {:?}", id, new_chapter);
    let mut conn = pool.get().map_err(internal_error)?;
    let res = conn
        .transaction(|conn| {
            diesel::update(chapters::table.filter(chapters::id.eq(id)))
                .set(new_chapter)
                .get_result(conn)
        })
        .map_err(internal_error)?;
    Ok(Json(res))
}

#[utoipa::path(
    delete,
    path = "/{id}",
    tag = "chapters",
    operation_id = "deleteChapter",
    description = "Delete a chapter by ID",
    params(("id" = i32, Path, description = "Chapter ID")),
    responses((status = 202, description = "The deleted chapter", body = Chapter))
)]
async fn delete_chapter(
    State(pool): State<AppPool>,
    Path(id): Path<i32>,
) -> Result<Json<Chapter>, (StatusCode, String)> {
    tracing::info!("Deleting chapter {:?}", id);
    let mut conn = pool.get().map_err(internal_error)?;
    let res = conn
        .transaction(|conn| {
            diesel::delete(chapters::table.filter(chapters::id.eq(id))).get_result(conn)
        })
        .map_err(internal_error)?;
    Ok(Json(res))
}
