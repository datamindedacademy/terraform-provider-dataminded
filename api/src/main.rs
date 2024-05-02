mod model;
mod routes;
mod schema;
use aide::{
    axum::{routing::get, ApiRouter, IntoApiResponse},
    openapi::{Info, OpenApi},
    redoc::Redoc,
    scalar::Scalar,
};
use routes::user::user_routes;

use axum::{Extension, Json};
use diesel::{connection::SimpleConnection, prelude::*};
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use std::net::SocketAddr;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use crate::routes::chapter::chapter_routes;
// normally part of your generated schema.rs file

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations/");

async fn serve_api(Extension(api): Extension<OpenApi>) -> impl IntoApiResponse {
    Json(api)
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
            "data/dm.db",
        ))
        .expect("Failed to create pool");

    // run the migrations on server startup
    {
        let mut conn = pool.get().unwrap();
        conn.transaction(|conn| conn.run_pending_migrations(MIGRATIONS).map(|_| ()))
            .unwrap();
        // enforce foreign key constraints
        conn.batch_execute("PRAGMA foreign_keys = ON").unwrap();
    }

    // build our application with some routes
    let app = ApiRouter::new()
        // generate redoc-ui using the openapi spec route
        .route(
            "/",
            Scalar::new("/api.json")
                .with_title("Data Minded example API")
                .axum_route(),
        )
        .route(
            "/redoc",
            Redoc::new("/api.json")
                .with_title("Data Minded example API")
                .axum_route(),
        )
        .nest("/user/", user_routes())
        .nest("/chapter/", chapter_routes())
        .with_state(pool)
        // We'll serve our generated document here.
        .route("/api.json", get(serve_api));

    let mut api = OpenApi {
        info: Info {
            description: Some("Data Minded example API".to_string()),
            ..Info::default()
        },
        ..OpenApi::default()
    };

    // run it with hyper
    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    tracing::debug!("listening on {addr}");
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(
        listener,
        app.finish_api(&mut api)
            .layer(Extension(api))
            .into_make_service(),
    )
    .await
    .unwrap();
}
