use serde::{Deserialize, Serialize};
use specta::Type;
use sqlx::prelude::FromRow;

#[derive(Debug, Serialize, Type)]
pub struct Kind {
    pub kind_id: i32,
    pub name: String,
}

#[derive(Debug, Serialize, Type)]
pub struct Producer {
    pub producer_id: i32,
    pub name: String,
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

#[derive(Debug, Serialize, Type, FromRow)]
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

// Request Types
#[derive(Debug, Deserialize, Type)]
pub struct SearchBeveragesRequest {
    pub query: String,
    pub sort: SortOptions,
}

#[derive(Debug, Deserialize, Type)]
#[serde(rename_all = "snake_case")]
pub enum SortDir {
    Asc,
    Desc,
}

impl ToString for SortDir {
    fn to_string(&self) -> String {
        match self {
            SortDir::Asc => "ASC".to_owned(),
            SortDir::Desc => "DESC".to_owned(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Type)]
#[serde(rename_all = "snake_case")]
pub enum SortColumn {
    BeverageId,
    Rating,
    Name,
    Kind,
}

impl ToString for SortColumn {
    fn to_string(&self) -> String {
        match self {
            SortColumn::BeverageId => "beverage_id".to_owned(),
            SortColumn::Rating => "rating".to_owned(),
            SortColumn::Name => "name".to_owned(),
            SortColumn::Kind => "kind".to_owned(),
        }
    }
}

#[derive(Debug, Deserialize, Type)]
pub struct SortOptions {
    pub column: SortColumn,
    pub direction: SortDir,
}
