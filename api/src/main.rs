mod db;
mod types;
mod util;

use std::sync::Arc;

use db::*;
use rspc::ErrorCode;
use serde::Deserialize;
use specta::Type;
use sqlx::{sqlite::SqliteConnectOptions, SqlitePool};
use tower_http::{cors::CorsLayer, services::ServeDir};
use types::InsertBeverage;
use util::*;

#[derive(Deserialize, Type)]
struct NameRequest {
    name: String,
}

async fn get_all_kinds(ctx: Context, _input: ()) -> Result<Vec<types::Kind>, rspc::Error> {
    ctx.db.get_all_kinds().await.anyhow_rspc()
}

async fn get_all_producers(ctx: Context, _input: ()) -> Result<Vec<types::Producer>, rspc::Error> {
    ctx.db.get_all_producers().await.anyhow_rspc()
}

async fn get_all_beverages(
    ctx: Context,
    _input: (),
) -> Result<Vec<types::JoinBeverage>, rspc::Error> {
    ctx.db.get_all_beverages().await.anyhow_rspc()
}

async fn add_kind(ctx: Context, input: NameRequest) -> Result<(), rspc::Error> {
    ctx.db.insert_kind(&input.name).await.anyhow_rspc()?;
    Ok(())
}

async fn add_producer(ctx: Context, input: NameRequest) -> Result<(), rspc::Error> {
    if input.name.is_empty() {
        return Err(rspc::Error::new(
            ErrorCode::BadRequest,
            "Name is empty".to_owned(),
        ));
    }
    ctx.db.insert_producer(&input.name).await.anyhow_rspc()?;
    Ok(())
}

#[derive(Deserialize, Type)]
struct UpdateBeverageRequest {
    beverage_id: Option<i32>,
    beverage: InsertBeverage,
}

async fn upsert_beverage(ctx: Context, input: UpdateBeverageRequest) -> Result<(), rspc::Error> {
    if let Some(beverage_id) = input.beverage_id {
        ctx.db
            .update_beverage(beverage_id, input.beverage)
            .await
            .anyhow_rspc()?;
    } else {
        ctx.db.insert_beverage(input.beverage).await.anyhow_rspc()?;
    }
    Ok(())
}

async fn delete_beverage(ctx: Context, input: i32) -> Result<(), rspc::Error> {
    ctx.db.delete_beverage(input as i64).await.anyhow_rspc()?;
    Ok(())
}

struct Context {
    db: Db,
}

fn router() -> rspc::Router<Context> {
    <rspc::Router<Context>>::new()
        .query("kind", |t| t(get_all_kinds))
        .query("producer", |t| t(get_all_producers))
        .query("beverage", |t| t(get_all_beverages))
        .mutation("kind", |t| t(add_kind))
        .mutation("producer", |t| t(add_producer))
        .mutation("beverage", |t| t(upsert_beverage))
        .mutation("deleteBeverage", |t| t(delete_beverage))
        .build()
}

#[tokio::main]
async fn main() {
    let (log_level, dist_dir) = if cfg!(debug_assertions) {
        (tracing::Level::DEBUG, "../ui/dist/")
    } else {
        (tracing::Level::INFO, "./dist")
    };

    tracing_subscriber::fmt().with_max_level(log_level).init();

    let pool = SqlitePool::connect_with(
        SqliteConnectOptions::new()
            .filename("alepal.db")
            .create_if_missing(true),
    )
    .await
    .expect("Unable to open database");

    let router = Arc::new(router());
    router
        .export_ts("../ui/src/api/rspc.ts")
        .expect("Unable to export typescript bindings");

    let db = Db::new(pool);

    let app = axum::Router::new()
        .with_state(db.clone())
        .nest_service("/", ServeDir::new(dist_dir))
        .nest("/rspc", rspc_axum::endpoint(router, || Context { db }))
        .layer(tower_http::trace::TraceLayer::new_for_http())
        .layer(
            CorsLayer::new()
                .allow_origin(tower_http::cors::Any)
                .allow_headers(tower_http::cors::Any)
                .allow_methods(tower_http::cors::Any),
        );

    let addr = "0.0.0.0:8080";
    println!("Listening on {addr}");
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
