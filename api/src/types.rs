use serde::{Deserialize, Serialize};
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

#[derive(Deserialize)]
pub struct InsertBeverage {
    pub name: String,
    pub producer_id: i64,
    pub kind_id: i64,
    pub rating: i64,
    pub description: String
}

#[derive(FromRow, Serialize)]
pub struct JoinBeverage {
    pub name: String,
    pub producer: String,
    pub kind: String,
    pub rating: i64,
    pub description: String
}

