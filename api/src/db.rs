use sqlx::SqlitePool;

use crate::types::*;

#[derive(Clone)]
pub struct Db {
    pool: SqlitePool,
}

impl Db {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }

    pub async fn get_all_kinds(&self) -> anyhow::Result<Vec<Kind>> {
        let record = sqlx::query!("SELECT * FROM kind")
            .fetch_all(&self.pool)
            .await?;

        Ok(record.into_iter().map(|r| Kind {
            kind_id: r.kind_id as i32,
            name: r.name,
        }).collect())
    }

    pub async fn insert_kind(&self, name: &str) -> anyhow::Result<i64> {
        let record = sqlx::query!("INSERT INTO kind (name) VALUES (?) RETURNING kind_id", name)
            .fetch_one(&self.pool)
            .await?;
        Ok(record.kind_id)
    }

    pub async fn get_all_producers(&self) -> anyhow::Result<Vec<Producer>> {
        let record = sqlx::query!("SELECT * FROM producer")
            .fetch_all(&self.pool)
            .await?;

        Ok(record.into_iter().map(|r| Producer {
            producer_id: r.producer_id as i32,
            name: r.name,
        }).collect())
    }

    pub async fn insert_producer(&self, name: &str) -> anyhow::Result<i64> {
        let record = sqlx::query!(
            "INSERT INTO producer (name) VALUES (?) RETURNING producer_id",
            name
        )
        .fetch_one(&self.pool)
        .await?;
        Ok(record.producer_id)
    }

    pub async fn insert_beverage(&self, beverage: InsertBeverage) -> anyhow::Result<i64> {
        let record = sqlx::query!("INSERT INTO beverage (name, kind_id, producer_id, rating, description) VALUES (?,?,?,?,?) RETURNING beverage_id", beverage.name, beverage.kind_id, beverage.producer_id, beverage.rating, beverage.description).fetch_one(&self.pool).await?;
        Ok(record.beverage_id)
    }

    pub async fn get_all_beverages(&self) -> anyhow::Result<Vec<JoinBeverage>> {
        let record = sqlx::query_as!(JoinBeverage, "SELECT beverage.name as name, kind.name as kind, producer.name as producer, beverage.rating as rating, beverage.description as description FROM beverage INNER JOIN kind ON kind.kind_id = beverage.kind_id INNER JOIN producer ON producer.producer_id = beverage.producer_id;")
            .fetch_all(&self.pool)
            .await?;
        Ok(record)
    }

    pub async fn delete_beverage(&self, beverage_id: i64) -> anyhow::Result<()> {
        sqlx::query!("DELETE FROM beverage WHERE beverage_id = ?", beverage_id)
            .execute(&self.pool)
            .await?;
        Ok(())
    }
}
