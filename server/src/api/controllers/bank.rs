use std::ops::Deref;

use anyhow::Context;
use poem_openapi::{param::Path, payload::Json, OpenApi};
use sqlx::{query, query_as, Pool, Sqlite};

use super::prelude::*;
use crate::models::{
    bank::{Bank, CreateBank},
    browse::Browse,
};

pub struct Api {
    db: Pool<Sqlite>,
}

pub fn api(db: &Pool<Sqlite>) -> Api {
    Api { db: db.clone() }
}

#[OpenApi(prefix_path = "/bank", tag = "super::Tags::Bank")]
impl Api {
    /// Get Bank
    #[oai(path = "/:id", method = "get")]
    async fn get(&self, id: Path<u32>) -> Result<Json<Bank>> {
        query_as::<_, Bank>("SELECT * FROM banks WHERE id = ?")
            .bind(*id)
            .fetch_optional(&self.db)
            .await
            .context("get bank")?
            .map(|o| Json(o))
            .ok_or(EntityNotExists("Bank").into())
    }

    /// Count Banks
    #[oai(path = "/count", method = "get")]
    async fn count(&self) -> Result<Json<u32>> {
        Ok(Json(
            query_as::<_, (u32,)>("SELECT COUNT(*) FROM banks")
                .fetch_one(&self.db)
                .await
                .context("count banks")?
                .0,
        ))
    }

    /// Browse Banks
    #[oai(path = "/browse", method = "post")]
    async fn browse(&self, data: Json<Browse>) -> Result<Json<Vec<Bank>>> {
        let data = data.deref();
        let skip = data.page_number() * data.count();
        Ok(Json(
            query_as::<_, Bank>("SELECT * FROM banks LIMIT ?, ?")
                .bind(skip)
                .bind(data.count())
                .fetch_all(&self.db)
                .await
                .context("browse banks")?,
        ))
    }

    /// Create Bank
    #[oai(path = "/", method = "post")]
    async fn create(&self, mut data: Json<CreateBank>) -> Result<Json<u32>> {
        data.validate()?;
        let result =
            query("INSERT INTO banks (country, city, zipcode, street, buildingNumber) VALUES (?, ?, ?, ?, ?)")
                .bind(&data.country)
                .bind(&data.city)
                .bind(&data.zipcode)
                .bind(&data.street)
                .bind(&data.building_number)
                .execute(&self.db)
                .await
                .context("insert bank")?;
        Ok(Json(result.last_insert_rowid() as u32))
    }

    /// Delete Bank
    #[oai(path = "/:id", method = "delete")]
    async fn delete(&self, id: Path<u32>) -> Result<()> {
        let result = query("DELETE FROM banks WHERE id = ?")
            .bind(*id)
            .execute(&self.db)
            .await
            .context("delete bank")?;
        if result.rows_affected() == 0 {
            Err(EntityNotExists("Bank").into())
        } else {
            Ok(())
        }
    }
}
