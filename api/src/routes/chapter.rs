use aide::axum::routing::get_with;
use aide::axum::ApiRouter;
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
        .api_route(
            "/",
            get_with(list_chapters, |op| {
                op.id("listChapters")
                    .description("List chapters")
                    .response_with::<200, Json<Vec<Chapter>>, _>(|res| {
                        res.description("A list of chapters").example(vec![Chapter {
                            id: 1,
                            name: "Chapter 1".to_string(),
                        }])
                    })
            })
            .post_with(create_chapter, |op| {
                op.id("createChapter")
                    .description("Create a new chapter")
                    .response_with::<201, Json<Chapter>, _>(|res| {
                        res.description("The created chapter").example(Chapter {
                            id: 1,
                            name: "Chapter 1".to_string(),
                        })
                    })
            }),
        )
        .api_route(
            "/:id",
            get_with(get_one_chapter, |op| {
                op.id("getChapter")
                    .description("Get a chapter by ID")
                    .response_with::<200, Json<Chapter>, _>(|res| {
                        res.description("The requested chapter").example(Chapter {
                            id: 1,
                            name: "Chapter 1".to_string(),
                        })
                    })
            })
            .put_with(update_chapter, |op| {
                op.id("updateChapter")
                    .description("Update a chapter by ID")
                    .response_with::<200, Json<Chapter>, _>(|res| {
                        res.description("The updated chapter").example(Chapter {
                            id: 1,
                            name: "Chapter 1".to_string(),
                        })
                    })
            })
            .delete_with(delete_chapter, |op| {
                op.id("deleteChapter")
                    .description("Delete a chapter by ID")
                    .response_with::<202, Json<Chapter>, _>(|res| {
                        res.description("The deleted chapter").example(Chapter {
                            id: 1,
                            name: "Chapter 1".to_string(),
                        })
                    })
            }),
        )
        .api_route(
            "/member",
            get_with(list_chapter_members, |op| {
                op.id("listChapterMembers")
                    .description("List chapter members")
                    .response_with::<200, Json<Vec<Chapter>>, _>(|res| {
                        res.description("A list of chapter members")
                            .example(vec![Chapter {
                                id: 1,
                                name: "Chapter 1".to_string(),
                            }])
                    })
            }),
        )
        .nest("/:id/member", chapter_member_routes())
}

async fn create_chapter(
    State(pool): State<diesel::r2d2::Pool<diesel::r2d2::ConnectionManager<SqliteConnection>>>,
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
