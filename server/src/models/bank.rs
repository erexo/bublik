use bublik_macros::Validation;
use poem_openapi::Object;
use serde::Deserialize;
use sqlx::FromRow;

#[derive(Object, Deserialize, FromRow)]
#[oai(rename_all = "camelCase", skip_serializing_if_is_none = true)]
#[serde(rename_all = "camelCase")]
#[sqlx(rename_all = "camelCase")]
pub struct Bank {
    pub id: u32,
    pub country: String,
    pub city: String,
    pub zipcode: String,
    pub street: String,
    pub building_number: String,
}

#[derive(Object, Validation)]
#[oai(rename_all = "camelCase", skip_serializing_if_is_none = true)]
#[val(trim, length = "field_length")]
pub struct CreateBank {
    pub country: String,
    pub city: String,
    pub zipcode: String,
    pub street: String,
    #[val(length = "building_field_length")]
    pub building_number: String,
}

fn field_length() -> (usize, usize) {
    (3, 64)
}

fn building_field_length() -> (usize, usize) {
    (1, 32)
}
