use std::i64;

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

        Ok(record
            .into_iter()
            .map(|r| Kind {
                kind_id: r.kind_id as i32,
                name: r.name,
            })
            .collect())
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

        Ok(record
            .into_iter()
            .map(|r| Producer {
                producer_id: r.producer_id as i32,
                name: r.name,
            })
            .collect())
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
        let record = sqlx::query!("INSERT INTO beverage (name, kind_id, producer_id, rating, description, image) VALUES (?,?,?,?,?,?) RETURNING beverage_id", beverage.name, beverage.kind_id, beverage.producer_id, beverage.rating, beverage.description, beverage.image).fetch_one(&self.pool).await?;
        Ok(record.beverage_id)
    }

    pub async fn get_all_beverages(&self) -> anyhow::Result<Vec<JoinBeverage>> {
        let record = sqlx::query!(
            r#"SELECT beverage_id,
                      beverage.name as name, 
                      kind.name as kind, 
                      producer.name as producer, 
                      beverage.producer_id,
                      beverage.kind_id,
                      rating, 
                      description, 
                      image 
               FROM beverage 
               INNER JOIN kind ON kind.kind_id = beverage.kind_id 
               INNER JOIN producer ON producer.producer_id = beverage.producer_id;"#
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(record
            .into_iter()
            .map(|r| JoinBeverage {
                beverage_id: r.beverage_id as i32,
                name: r.name,
                producer_id: r.producer_id as i32,
                producer: r.producer,
                kind_id: r.kind_id as i32,
                kind: r.kind,
                rating: r.rating,
                description: r.description,
                image: r.image,
            })
            .collect())
    }

    pub async fn update_beverage(
        &self,
        beverage_id: i32,
        beverage: InsertBeverage,
    ) -> anyhow::Result<()> {
        sqlx::query!(
            r#"UPDATE beverage
               SET name = ?,
                   producer_id = ?,
                   kind_id = ?,
                   rating = ?,
                   description = ?,
                   image = ? 
               WHERE beverage_id = ?;"#,
            beverage.name,
            beverage.producer_id,
            beverage.kind_id,
            beverage.rating,
            beverage.description,
            beverage.image,
            beverage_id,
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    pub async fn delete_beverage(&self, beverage_id: i64) -> anyhow::Result<()> {
        sqlx::query!("DELETE FROM beverage WHERE beverage_id = ?", beverage_id)
            .execute(&self.pool)
            .await?;
        Ok(())
    }

    pub async fn delete_kind(&self, kind_id: i64) -> anyhow::Result<()> {
        sqlx::query!("DELETE FROM kind WHERE kind_id = ?", kind_id)
            .execute(&self.pool)
            .await?;
        Ok(())
    }

    pub async fn delete_producer(&self, producer_id: i64) -> anyhow::Result<()> {
        sqlx::query!("DELETE FROM producer WHERE producer_id = ?", producer_id)
            .execute(&self.pool)
            .await?;
        Ok(())
    }

    pub async fn search_beverages(
        &self,
        search_term: String,
    ) -> anyhow::Result<Vec<JoinBeverage>> {
        let record = sqlx::query!(
            r#"SELECT beverage_id,
                      beverage.name as name, 
                      kind.name as kind, 
                      producer.name as producer, 
                      beverage.producer_id,
                      beverage.kind_id,
                      rating, 
                      description, 
                      image 
               FROM beverage 
               INNER JOIN kind ON kind.kind_id = beverage.kind_id 
               INNER JOIN producer ON producer.producer_id = beverage.producer_id
               WHERE beverage_id IN (SELECT beverage_id FROM beverage_search WHERE beverage_search MATCH ?)"#,
            search_term
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(record
            .into_iter()
            .map(|r| JoinBeverage {
                beverage_id: r.beverage_id as i32,
                name: r.name,
                producer_id: r.producer_id as i32,
                producer: r.producer,
                kind_id: r.kind_id as i32,
                kind: r.kind,
                rating: r.rating,
                description: r.description,
                image: r.image,
            })
            .collect())
    }
}
