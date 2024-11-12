mod api;
mod db;
mod types;
mod util;

use anyhow::Result;
use api::*;
use db::*;
use sqlx::{sqlite::SqliteConnectOptions, SqlitePool};
use tower_http::{cors::CorsLayer, services::ServeDir};

use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(long, default_value = "0.0.0.0:8080")]
    address: String,

    #[arg(long, default_value_t = tracing::Level::DEBUG)]
    tracing_level: tracing::Level,

    #[arg(long, default_value_t = false)]
    run_migrations: bool,
}

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let args = Args::parse();

    let dist_dir = if cfg!(debug_assertions) {
        "../ui/dist/"
    } else {
        "./dist"
    };

    tracing_subscriber::fmt()
        .with_max_level(args.tracing_level)
        .init();

    let pool = SqlitePool::connect_with(
        SqliteConnectOptions::new()
            .filename("alepal.db")
            .create_if_missing(true),
    )
    .await
    .expect("Unable to open database");

    if args.run_migrations {
        println!("Running migrations");
        sqlx::migrate!("./migrations")
            .run(&pool)
            .await
            .expect("Unable to run migrations")
    }

    let router = rspc_router().arced();
    if cfg!(debug_assertions) {
        router
            .export_ts("../ui/src/api/rspc.ts")
            .expect("Unable to export typescript bindings");
    }

    let db = Db::new(pool);

    let app = axum::Router::new()
        .with_state(db.clone())
        .nest_service("/", ServeDir::new(dist_dir))
        .nest("/rspc", rspc_axum::endpoint(router, || RspcContext { db }))
        .layer(tower_http::trace::TraceLayer::new_for_http())
        .layer(
            CorsLayer::new()
                .allow_origin(tower_http::cors::Any)
                .allow_headers(tower_http::cors::Any)
                .allow_methods(tower_http::cors::Any),
        );

    println!("Listening on {}", args.address);
    let listener = tokio::net::TcpListener::bind(args.address).await.unwrap();
    axum::serve(listener, app).await
}
