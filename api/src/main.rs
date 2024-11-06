mod db;
mod types;
mod util;

use actix_cors::Cors;
use db::*;
use serde::Deserialize;
use sqlx::{sqlite::SqliteConnectOptions, SqlitePool};
use types::InsertBeverage;
use util::*;

use actix_web::{
    delete, get, post, web::{self, Json, JsonConfig}, App, HttpResponse, HttpServer, Responder
};

#[derive(Deserialize)]
struct NameRequest {
    name: String
}

#[get("/producer")]
async fn get_all_producers(db: web::Data<Db>) -> impl Responder {
    db.get_all_producers().await.to_response()
}

#[post("/producer")]
async fn post_producer(req_body: Json<NameRequest>, db: web::Data<Db>) -> HttpResponse {
    let name = &req_body.name;
    if name.is_empty() {
        return HttpResponse::BadRequest().body("Empty Name");
    }
    db.insert_producer(&name).await.to_response()
}

#[get("/kind")]
async fn get_all_kinds(db: web::Data<Db>) -> impl Responder {
    db.get_all_kinds().await.to_response()
}

#[post("/kind")]
async fn post_kind(req_body: Json<NameRequest>, db: web::Data<Db>) -> impl Responder {
    db.insert_kind(&req_body.name).await.to_response()
}

#[get("/beverage")]
async fn get_beverage(db: web::Data<Db>) -> impl Responder {
    db.get_all_beverages().await.to_response()
}

#[post("/beverage")]
async fn post_beverage(body: Json<InsertBeverage>, db: web::Data<Db>) -> impl Responder {
    db.insert_beverage(&body.name, body.kind_id, body.producer_id).await.to_response()
}

#[delete("/beverage/{id}")]
async fn delete_beverage(path: web::Path<i64>, db: web::Data<Db>) -> impl Responder {
    db.delete_beverage(path.into_inner()).await.to_response()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let pool = SqlitePool::connect_with(
        SqliteConnectOptions::new()
            .filename("alepal.db")
            .create_if_missing(true),
    )
    .await
    .expect("Unable to open database");

    let db = Db::new(pool);

    HttpServer::new(move || {
        App::new()
            .wrap(Cors::permissive())
            .app_data(web::Data::new(db.clone()))
            .service(get_all_kinds)
            .service(get_all_producers)
            .service(post_kind)
            .service(post_producer)
            .service(get_beverage)
            .service(post_beverage)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
