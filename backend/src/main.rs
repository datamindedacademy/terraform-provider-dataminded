mod schema;
use schema::{chapter_members, chapters, users};

use axum::{
    extract::State,
    http::StatusCode,
    response::Json,
    routing::{get, post},
    Router,
};
use diesel::prelude::*;
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use std::net::SocketAddr;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

// normally part of your generated schema.rs file

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations/");

#[derive(serde::Serialize, Selectable, Queryable)]
struct User {
    id: i32,
    name: String,
}

#[derive(serde::Deserialize, Insertable)]
#[diesel(table_name = users)]
struct NewUser {
    name: String,
}

#[derive(serde::Serialize, Selectable, Queryable)]
struct Chapter {
    id: i32,
    name: String,
}

#[derive(serde::Deserialize, Insertable)]
#[diesel(table_name = chapters)]
struct NewChapter {
    name: String,
}

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "data-minded-api=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    // set up connection pool
    let pool = diesel::r2d2::Pool::builder()
        .max_size(1)
        .build(diesel::r2d2::ConnectionManager::<SqliteConnection>::new(
            ":memory:",
        ))
        .expect("Failed to create pool");

    // run the migrations on server startup
    {
        let mut conn = pool.get().unwrap();
        conn.transaction(|conn| conn.run_pending_migrations(MIGRATIONS).map(|_| ()))
            .unwrap();
    }

    // build our application with some routes
    let app = Router::new()
        .route("/user/list", get(list_users))
        .route("/user/create", post(create_user))
        .with_state(pool);

    // run it with hyper
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::debug!("listening on {addr}");
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn create_user(
    State(pool): State<diesel::r2d2::Pool<diesel::r2d2::ConnectionManager<SqliteConnection>>>,
    Json(new_user): Json<NewUser>,
) -> Result<Json<usize>, (StatusCode, String)> {
    let mut conn = pool.get().map_err(internal_error)?;
    let res = conn
        .transaction(|conn| {
            diesel::insert_into(users::table)
                .values(new_user)
                .execute(conn)
        })
        .map_err(internal_error)?;
    Ok(Json(res))
}

async fn list_users(
    State(pool): State<diesel::r2d2::Pool<diesel::r2d2::ConnectionManager<SqliteConnection>>>,
) -> Result<Json<Vec<User>>, (StatusCode, String)> {
    let mut conn = pool.get().map_err(internal_error)?;
    let res = conn
        .transaction(|conn| users::table.select(User::as_select()).load(conn))
        .map_err(internal_error)?;
    Ok(Json(res))
}

/// Utility function for mapping any error into a `500 Internal Server Error`
/// response.
fn internal_error<E>(err: E) -> (StatusCode, String)
where
    E: std::error::Error,
{
    (StatusCode::INTERNAL_SERVER_ERROR, err.to_string())
}
