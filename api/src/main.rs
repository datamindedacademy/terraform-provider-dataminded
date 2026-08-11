mod model;
mod routes;
mod schema;

use axum::Json;
use diesel::{connection::SimpleConnection, prelude::*};
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use std::net::SocketAddr;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use utoipa::OpenApi;
use utoipa_axum::router::OpenApiRouter;
use utoipa_redoc::{Redoc, Servable as RedocServable};
use utoipa_scalar::{Scalar, Servable as ScalarServable};

use crate::routes::chapter::chapter_routes;
use crate::routes::user::user_routes;

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations/");

#[derive(OpenApi)]
#[openapi(info(
    title = "Dataminded example API",
    description = "Dataminded example API"
))]
struct ApiDoc;

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "data-minded-api=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let pool = diesel::r2d2::Pool::builder()
        .max_size(1)
        .build(diesel::r2d2::ConnectionManager::<SqliteConnection>::new(
            "data/dm.db",
        ))
        .expect("Failed to create pool");

    {
        let mut conn = pool.get().unwrap();
        conn.transaction(|conn| conn.run_pending_migrations(MIGRATIONS).map(|_| ()))
            .unwrap();
        conn.batch_execute("PRAGMA foreign_keys = ON").unwrap();
    }

    let (router, api) = OpenApiRouter::with_openapi(ApiDoc::openapi())
        .nest("/user", user_routes())
        .nest("/chapter", chapter_routes())
        .with_state(pool)
        .split_for_parts();

    let app = router
        .merge(Scalar::with_url("/", api.clone()))
        .merge(Redoc::with_url("/redoc", api.clone()))
        .route("/api.json", axum::routing::get(move || async move { Json(api) }));

    let port: u16 = std::env::var("PORT")
        .ok()
        .and_then(|p| p.parse().ok())
        .unwrap_or(3000);
    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    tracing::debug!("listening on {addr}");
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
