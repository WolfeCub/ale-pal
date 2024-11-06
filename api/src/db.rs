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
        let record = sqlx::query_as!(Kind, "SELECT * FROM kind")
            .fetch_all(&self.pool)
            .await?;
        Ok(record)
    }

    pub async fn insert_kind(&self, name: &str) -> anyhow::Result<i64> {
        let record = sqlx::query!("INSERT INTO kind (name) VALUES (?) RETURNING kind_id", name)
            .fetch_one(&self.pool)
            .await?;
        Ok(record.kind_id)
    }

    pub async fn get_all_producers(&self) -> anyhow::Result<Vec<Producer>> {
        let record = sqlx::query_as!(Producer, "SELECT * FROM producer")
            .fetch_all(&self.pool)
            .await?;
        Ok(record)
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

    pub async fn insert_beverage(
        &self,
        name: &str,
        kind_id: i64,
        producer_id: i64,
    ) -> anyhow::Result<i64> {
        let record = sqlx::query!("INSERT INTO beverage (name, kind_id, producer_id) VALUES (?,?,?) RETURNING beverage_id", name, kind_id, producer_id).fetch_one(&self.pool).await?;
        Ok(record.beverage_id)
    }

    pub async fn get_all_beverages(&self) -> anyhow::Result<Vec<JoinBeverage>> {
        let record = sqlx::query_as!(JoinBeverage, "SELECT beverage.name as name, kind.name as kind, producer.name as producer FROM beverage INNER JOIN kind ON kind.kind_id = beverage.kind_id INNER JOIN producer ON producer.producer_id = beverage.producer_id;")
            .fetch_all(&self.pool)
            .await?;
        Ok(record)
    }

    pub async fn delete_beverage(&self, beverage_id: i64) -> anyhow::Result<()> {
        sqlx::query!("DELETE FROM beverage WHERE beverage_id = ?", beverage_id).execute(&self.pool).await?;
        Ok(())
    }
}
