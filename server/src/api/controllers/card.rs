use std::ops::Deref;

use anyhow::Context;
use poem_openapi::{param::Path, payload::Json, OpenApi};
use sqlx::{error::ErrorKind, query, query_as, Pool, Sqlite};

use super::prelude::*;
use crate::models::{
    browse::Browse,
    card::{Card, CreateCard},
};

pub struct Api {
    db: Pool<Sqlite>,
}

pub fn api(db: &Pool<Sqlite>) -> Api {
    Api { db: db.clone() }
}

#[OpenApi(prefix_path = "/card", tag = "super::Tags::Card")]
impl Api {
    /// Get Card
    #[oai(path = "/:id", method = "get")]
    async fn get(&self, id: Path<u32>) -> Result<Json<Card>> {
        query_as::<_, Card>("SELECT * FROM cards WHERE id = ?")
            .bind(*id)
            .fetch_optional(&self.db)
            .await
            .context("get card")?
            .map(|o| Json(o))
            .ok_or(EntityNotExists("Card").into())
    }

    /// Count Cards
    #[oai(path = "/count", method = "get")]
    async fn count(&self) -> Result<Json<u32>> {
        Ok(Json(
            query_as::<_, (u32,)>("SELECT COUNT(*) FROM cards")
                .fetch_one(&self.db)
                .await
                .context("count cards")?
                .0,
        ))
    }

    /// Browse Cards
    #[oai(path = "/browse", method = "post")]
    async fn browse(&self, data: Json<Browse>) -> Result<Json<Vec<Card>>> {
        let data = data.deref();
        let skip = data.page_number() * data.count();
        Ok(Json(
            query_as::<_, Card>("SELECT * FROM cards LIMIT ?, ?")
                .bind(skip)
                .bind(data.count())
                .fetch_all(&self.db)
                .await
                .context("browse cards")?,
        ))
    }

    /// Create Card
    #[oai(path = "/", method = "post")]
    async fn create(&self, mut data: Json<CreateCard>) -> Result<Json<u32>> {
        data.validate()?;
        let result =
            query("INSERT INTO cards (cardType, number, expiration, owner) VALUES (?, ?, ?, ?)")
                .bind(data.card_type as u32)
                .bind(&data.number)
                .bind(&data.expiration)
                .bind(&data.owner)
                .execute(&self.db)
                .await;
        if let Err(err) = &result {
            if let Some(err) = err.as_database_error() {
                if err.kind() == ErrorKind::UniqueViolation {
                    return Err(CardNumberAlreadyExists(data.number.clone()).into());
                }
            }
        }
        let result = result.context("insert card")?;
        Ok(Json(result.last_insert_rowid() as u32))
    }

    /// Delete Card
    #[oai(path = "/:id", method = "delete")]
    async fn delete(&self, id: Path<u32>) -> Result<()> {
        let result = query("DELETE FROM cards WHERE id = ?")
            .bind(*id)
            .execute(&self.db)
            .await
            .context("delete card")?;
        if result.rows_affected() == 0 {
            Err(EntityNotExists("Card").into())
        } else {
            Ok(())
        }
    }
}
