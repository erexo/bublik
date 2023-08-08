use bublik_macros::Validation;
use chrono::NaiveDate;
use int_enum::IntEnum;
use poem_openapi::{Enum, Object};
use serde::Deserialize;
use sqlx::FromRow;

#[derive(Object, Deserialize, FromRow)]
#[oai(rename_all = "camelCase", skip_serializing_if_is_none = true)]
#[serde(rename_all = "camelCase")]
#[sqlx(rename_all = "camelCase")]
pub struct User {
    pub id: u32,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub phone: String,
    #[oai(default = "NaiveDate::default")]
    pub birthday: NaiveDate,
    #[sqlx(try_from = "u32")]
    pub user_type: UserType,
}

#[repr(u32)]
#[derive(Enum, Deserialize, Clone, Copy, IntEnum)]
pub enum UserType {
    Worker = 1,
    Manager = 2,
    Customer = 3,
}

impl From<u32> for UserType {
    fn from(value: u32) -> Self {
        Self::from_int(value).unwrap_or(Self::Worker)
    }
}

#[derive(Object, Validation)]
#[oai(rename_all = "camelCase", skip_serializing_if_is_none = true)]
#[val(trim, length = "field_length")]
pub struct CreateUser {
    pub first_name: String,
    pub last_name: String,
    #[val(
        pattern = r"^([a-z0-9_+]([a-z0-9_+.]*[a-z0-9_+])?)@([a-z0-9]+([\-\.]{1}[a-z0-9]+)*\.[a-z]{2,6})"
    )]
    pub email: String,
    pub phone: String,
    #[oai(default = "NaiveDate::default")]
    pub birthday: NaiveDate,
    pub user_type: UserType,
}

fn field_length() -> (usize, usize) {
    (3, 64)
}
