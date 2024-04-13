mod model;
mod routes;
mod schema;
use routes::user::user_routes;

use axum::Router;
use diesel::prelude::*;
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use std::net::SocketAddr;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use crate::routes::{chapter::chapter_routes, chapter_member::chapter_member_routes};
// normally part of your generated schema.rs file

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations/");

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
        .nest("/user", user_routes())
        .nest("/chapter", chapter_routes())
        .nest("/chapter/member", chapter_member_routes())
        .with_state(pool);

    // run it with hyper
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::debug!("listening on {addr}");
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
