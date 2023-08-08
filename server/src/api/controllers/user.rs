use std::ops::Deref;

use anyhow::Context;
use poem_openapi::{param::Path, payload::Json, OpenApi};
use sqlx::{error::ErrorKind, query, query_as, Pool, Sqlite};

use super::prelude::*;
use crate::models::{
    browse::Browse,
    user::{CreateUser, User},
};

pub struct Api {
    db: Pool<Sqlite>,
}

pub fn api(db: &Pool<Sqlite>) -> Api {
    Api { db: db.clone() }
}

#[OpenApi(prefix_path = "/user", tag = "super::Tags::User")]
impl Api {
    /// Get User
    #[oai(path = "/:id", method = "get")]
    async fn get(&self, id: Path<u32>) -> Result<Json<User>> {
        query_as::<_, User>("SELECT * FROM users WHERE id = ?")
            .bind(*id)
            .fetch_optional(&self.db)
            .await
            .context("get user")?
            .map(|o| Json(o))
            .ok_or(EntityNotExists("User").into())
    }

    /// Count Users
    #[oai(path = "/count", method = "get")]
    async fn count(&self) -> Result<Json<u32>> {
        Ok(Json(
            query_as::<_, (u32,)>("SELECT COUNT(*) FROM users")
                .fetch_one(&self.db)
                .await
                .context("count users")?
                .0,
        ))
    }

    /// Browse Users
    #[oai(path = "/browse", method = "post")]
    async fn browse(&self, data: Json<Browse>) -> Result<Json<Vec<User>>> {
        let data = data.deref();
        let skip = data.page_number() * data.count();
        Ok(Json(
            query_as::<_, User>("SELECT * FROM users LIMIT ?, ?")
                .bind(skip)
                .bind(data.count())
                .fetch_all(&self.db)
                .await
                .context("browse users")?,
        ))
    }

    /// Create User
    #[oai(path = "/", method = "post")]
    async fn create(&self, mut data: Json<CreateUser>) -> Result<Json<u32>> {
        data.validate()?;
        let result = query("INSERT INTO users (firstName, lastName, email, phone, birthday, userType) VALUES (?, ?, ?, ?, ?, ?)")
            .bind(&data.first_name)
            .bind(&data.last_name)
            .bind(&data.email)
            .bind(&data.phone)
            .bind(&data.birthday)
            .bind(data.user_type as u32)
            .execute(&self.db)
            .await;
        if let Err(err) = &result {
            if let Some(err) = err.as_database_error() {
                if err.kind() == ErrorKind::UniqueViolation {
                    return Err(UserEmailAlreadyExists(data.email.clone()).into());
                }
            }
        }
        let result = result.context("insert user")?;
        Ok(Json(result.last_insert_rowid() as u32))
    }

    /// Delete User
    #[oai(path = "/:id", method = "delete")]
    async fn delete(&self, id: Path<u32>) -> Result<()> {
        let result = query("DELETE FROM users WHERE id = ?")
            .bind(*id)
            .execute(&self.db)
            .await
            .context("delete user")?;
        if result.rows_affected() == 0 {
            Err(EntityNotExists("User").into())
        } else {
            Ok(())
        }
    }
}
