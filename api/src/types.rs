use serde::Serialize;
use sqlx::FromRow;

#[derive(FromRow, Serialize)]
pub struct Kind {
    pub kind_id: i64,
    pub name: String
}

#[derive(FromRow, Serialize)]
pub struct Producer {
    pub producer_id: i64,
    pub name: String
}
