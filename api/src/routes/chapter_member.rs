use aide::axum::routing::get_with;
use aide::axum::ApiRouter;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::Json;
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool};
use diesel::SqliteConnection;

use super::error::internal_error;
use crate::model::chapter_member::{ChapterMember, ChapterRole, NewChapterMember};
use crate::schema::chapter_members;

pub fn chapter_member_routes() -> ApiRouter<Pool<ConnectionManager<SqliteConnection>>> {
    ApiRouter::new()
        .api_route(
            "/",
            get_with(list_chapter_members, |op| {
                op.id("listChapterMembers")
                    .description("List chapter members")
                    .response_with::<200, Json<Vec<ChapterMember>>, _>(|res| {
                        res.description("A list of chapter members")
                            .example(vec![ChapterMember {
                                chapter_id: 1,
                                user_id: 1,
                                role: Some(ChapterRole::Contributor),
                            }])
                    })
            }),
        )
        .api_route(
            "/:user_id",
            get_with(get_one_chapter_member, |op| {
                op.id("getChapterMember")
                    .description("Get a chapter member by ID")
                    .response_with::<200, Json<ChapterMember>, _>(|res| {
                        res.description("The requested chapter member")
                            .example(ChapterMember {
                                chapter_id: 1,
                                user_id: 1,
                                role: Some(ChapterRole::Contributor),
                            })
                    })
            })
            .post_with(create_chapter_member, |op| {
                op.id("createChapterMember")
                    .description("Create a new chapter member")
                    .response_with::<201, Json<ChapterMember>, _>(|res| {
                        res.description("The created chapter member")
                            .example(ChapterMember {
                                chapter_id: 1,
                                user_id: 1,
                                role: Some(ChapterRole::Contributor),
                            })
                    })
            })
            .put_with(update_chapter_member, |op| {
                op.id("updateChapterMember")
                    .description("Update a chapter member by ID")
                    .response_with::<200, Json<ChapterMember>, _>(|res| {
                        res.description("The updated chapter member")
                            .example(ChapterMember {
                                chapter_id: 1,
                                user_id: 1,
                                role: Some(ChapterRole::Contributor),
                            })
                    })
            })
            .delete_with(delete_chapter_member, |op| {
                op.id("deleteChapterMember")
                    .description("Delete a chapter member by ID")
                    .response_with::<202, Json<ChapterMember>, _>(|res| {
                        res.description("The deleted chapter member")
                            .example(ChapterMember {
                                chapter_id: 1,
                                user_id: 1,
                                role: Some(ChapterRole::Contributor),
                            })
                    })
            }),
        )
}

async fn create_chapter_member(
    State(pool): State<diesel::r2d2::Pool<diesel::r2d2::ConnectionManager<SqliteConnection>>>,
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
    tracing::info!(
        "Changing member {:?} of chapter {:?} to role {:?}",
        user_id,
        chapter_id,
        chapter_member.role
    );

    let mut chapter_member = chapter_member; // Declare chapter_member as mutable
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

async fn delete_chapter_member(
    State(pool): State<diesel::r2d2::Pool<diesel::r2d2::ConnectionManager<SqliteConnection>>>,
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

async fn get_one_chapter_member(
    State(pool): State<diesel::r2d2::Pool<diesel::r2d2::ConnectionManager<SqliteConnection>>>,
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
