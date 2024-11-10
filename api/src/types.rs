use serde::{Deserialize, Serialize};
use specta::Type;

#[derive(Debug, Serialize, Type)]
pub struct Kind {
    pub kind_id: i32,
    pub name: String
}

#[derive(Debug, Serialize, Type)]
pub struct Producer {
    pub producer_id: i32,
    pub name: String
}

#[derive(Debug, Deserialize, Type)]
pub struct InsertBeverage {
    pub name: String,
    pub producer_id: i32,
    pub kind_id: i32,
    pub rating: f64,
    pub description: String,
    pub image: Option<Vec<u8>>,
}

#[derive(Debug, Serialize, Type)]
pub struct JoinBeverage {
    pub beverage_id: i32,
    pub name: String,
    pub producer_id: i32,
    pub producer: String,
    pub kind_id: i32,
    pub kind: String,
    pub rating: f64,
    pub description: String,
    pub image: Option<Vec<u8>>,
}

