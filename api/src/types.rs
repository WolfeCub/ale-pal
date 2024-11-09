use serde::{Deserialize, Serialize};
use specta::Type;
use sqlx::FromRow;

#[derive(FromRow, Serialize, Type)]
pub struct Kind {
    pub kind_id: i32,
    pub name: String
}

#[derive(FromRow, Serialize, Type)]
pub struct Producer {
    pub producer_id: i32,
    pub name: String
}

#[derive(Deserialize, Type)]
pub struct InsertBeverage {
    pub name: String,
    pub producer_id: i32,
    pub kind_id: i32,
    pub rating: i32,
    pub description: String,
    pub image: Option<Vec<u8>>,
}

#[derive(FromRow, Serialize, Type)]
pub struct JoinBeverage {
    pub name: String,
    pub producer: String,
    pub kind: String,
    pub rating: f64,
    pub description: String,
    pub image: Option<Vec<u8>>,
}

