use aide::axum::routing::get;
use aide::axum::ApiRouter;
use aide::axum::IntoApiResponse;
use axum::extract::Path;
use axum::extract::State;
use axum::http::StatusCode;
use axum::Json;
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool};
use diesel::SqliteConnection;

use super::chapter_member::chapter_member_routes;
use super::chapter_member::list_chapter_members;
use super::error::internal_error;
use crate::model::chapter::{Chapter, NewChapter};
use crate::schema::chapters;

pub fn chapter_routes() -> ApiRouter<Pool<ConnectionManager<SqliteConnection>>> {
    ApiRouter::new()
        .route("/", get(list_chapters).post(create_chapter))
        .route(
            "/:id",
            get(get_one_chapter)
                .put(update_chapter)
                .delete(delete_chapter),
        )
        .route("/member", get(list_chapter_members))
        .nest("/:id/member", chapter_member_routes())
}

async fn create_chapter(
    State(pool): State<diesel::r2d2::Pool<diesel::r2d2::ConnectionManager<SqliteConnection>>>,
    Json(new_chapter): Json<NewChapter>,
) -> Result<impl IntoApiResponse, (StatusCode, String)> {
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

async fn list_chapters(
    State(pool): State<diesel::r2d2::Pool<diesel::r2d2::ConnectionManager<SqliteConnection>>>,
) -> Result<Json<Vec<Chapter>>, (StatusCode, String)> {
    tracing::info!("Listing chapters");
    let mut conn = pool.get().map_err(internal_error)?;
    let res = conn
        .transaction(|conn| chapters::table.select(Chapter::as_select()).load(conn))
        .map_err(internal_error)?;
    Ok(Json(res))
}

async fn delete_chapter(
    State(pool): State<diesel::r2d2::Pool<diesel::r2d2::ConnectionManager<SqliteConnection>>>,
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

async fn update_chapter(
    State(pool): State<diesel::r2d2::Pool<diesel::r2d2::ConnectionManager<SqliteConnection>>>,
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

async fn get_one_chapter(
    State(pool): State<diesel::r2d2::Pool<diesel::r2d2::ConnectionManager<SqliteConnection>>>,
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
