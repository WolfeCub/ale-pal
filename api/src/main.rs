mod db;
mod util;
mod types;

use sqlx::{sqlite::SqliteConnectOptions, SqlitePool};
use db::*;
use util::*;

use actix_web::{
    get, post,
    web::{self, Json},
    App, HttpServer, Responder,
};
use serde::Deserialize;

#[get("/kind")]
async fn get_all_kinds(db: web::Data<Db>) -> impl Responder {
    db.get_all_kinds().await.to_responder()
}

#[derive(Deserialize)]
struct PostKindRequest {
    name: String
}

#[post("/kind")]
async fn post_kind(req_body: Json<PostKindRequest>, db: web::Data<Db>) -> impl Responder {
    db.insert_kind(&req_body.name).await.to_responder()
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
            .app_data(web::Data::new(db.clone()))
            .service(post_kind)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
