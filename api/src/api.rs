use rspc::ErrorCode;
use serde::Deserialize;
use specta::Type;

use crate::db::*;
use crate::types;
use crate::types::InsertBeverage;
use crate::util::*;

#[derive(Deserialize, Type)]
struct NameRequest {
    name: String,
}

async fn get_all_kinds(ctx: RspcContext, _input: ()) -> Result<Vec<types::Kind>, rspc::Error> {
    ctx.db.get_all_kinds().await.anyhow_rspc()
}

async fn get_all_producers(
    ctx: RspcContext,
    _input: (),
) -> Result<Vec<types::Producer>, rspc::Error> {
    ctx.db.get_all_producers().await.anyhow_rspc()
}

#[derive(Deserialize, Type)]
struct SearchBeveragesRequest {
    query: String,
}

async fn get_all_beverages(
    ctx: RspcContext,
    input: SearchBeveragesRequest,
) -> Result<Vec<types::JoinBeverage>, rspc::Error> {
    if input.query.is_empty() {
        ctx.db.get_all_beverages().await.anyhow_rspc()
    } else {
        ctx.db.search_beverages(input.query).await.anyhow_rspc()
    }
}

async fn add_kind(ctx: RspcContext, input: NameRequest) -> Result<i32, rspc::Error> {
    let id = ctx.db.insert_kind(&input.name).await.anyhow_rspc()?;
    Ok(id as i32)
}

async fn add_producer(ctx: RspcContext, input: NameRequest) -> Result<i32, rspc::Error> {
    if input.name.is_empty() {
        return Err(rspc::Error::new(
            ErrorCode::BadRequest,
            "Name is empty".to_owned(),
        ));
    }
    let id = ctx.db.insert_producer(&input.name).await.anyhow_rspc()?;
    Ok(id as i32)
}

#[derive(Deserialize, Type)]
struct UpdateBeverageRequest {
    beverage_id: Option<i32>,
    beverage: InsertBeverage,
}

async fn upsert_beverage(
    ctx: RspcContext,
    input: UpdateBeverageRequest,
) -> Result<(), rspc::Error> {
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

async fn delete_beverage(ctx: RspcContext, input: i32) -> Result<(), rspc::Error> {
    ctx.db.delete_beverage(input as i64).await.anyhow_rspc()?;
    Ok(())
}

async fn delete_kind(ctx: RspcContext, input: i32) -> Result<(), rspc::Error> {
    ctx.db.delete_kind(input as i64).await.anyhow_rspc()?;
    Ok(())
}

async fn delete_producer(ctx: RspcContext, input: i32) -> Result<(), rspc::Error> {
    ctx.db.delete_producer(input as i64).await.anyhow_rspc()?;
    Ok(())
}

pub struct RspcContext {
    pub db: Db,
}

pub fn rspc_router() -> rspc::Router<RspcContext> {
    <rspc::Router<RspcContext>>::new()
        .query("kind", |t| t(get_all_kinds))
        .query("producer", |t| t(get_all_producers))
        .query("beverage", |t| t(get_all_beverages))
        .mutation("kind", |t| t(add_kind))
        .mutation("producer", |t| t(add_producer))
        .mutation("beverage", |t| t(upsert_beverage))
        .mutation("deleteBeverage", |t| t(delete_beverage))
        .mutation("deleteKind", |t| t(delete_kind))
        .mutation("deleteProducer", |t| t(delete_producer))
        .build()
}
