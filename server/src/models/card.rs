use bublik_macros::Validation;
use int_enum::IntEnum;
use poem_openapi::{Enum, Object};
use serde::Deserialize;
use sqlx::FromRow;

#[derive(Object, Deserialize, FromRow)]
#[oai(rename_all = "camelCase", skip_serializing_if_is_none = true)]
#[serde(rename_all = "camelCase")]
#[sqlx(rename_all = "camelCase")]
pub struct Card {
    pub id: u32,
    #[sqlx(try_from = "u32")]
    #[serde(rename = "type")]
    pub card_type: CardType,
    pub number: String,
    pub expiration: String,
    pub owner: String,
}

#[repr(u32)]
#[derive(Enum, Deserialize, Clone, Copy, IntEnum)]
pub enum CardType {
    Visa = 1,
    #[serde(rename = "Visa Retired")]
    #[oai(rename = "Visa Retired")]
    VisaRetired = 2,
    MasterCard = 3,
    #[serde(rename = "Discover Card")]
    #[oai(rename = "Discover Card")]
    DiscoverCard = 4,
    #[serde(rename = "American Express")]
    #[oai(rename = "American Express")]
    AmericanExpress = 5,
}

impl From<u32> for CardType {
    fn from(value: u32) -> Self {
        Self::from_int(value).unwrap_or(Self::Visa)
    }
}

#[derive(Object, Validation)]
#[oai(rename_all = "camelCase", skip_serializing_if_is_none = true)]
#[val(trim, length = "field_length")]
pub struct CreateCard {
    pub card_type: CardType,
    #[val(pattern = r"^[0-9]*$")]
    pub number: String,
    #[val(pattern = r"^(0[1-9]|1[0-2])\/?([0-9]{4}|[0-9]{2})$")]
    pub expiration: String,
    pub owner: String,
}

fn field_length() -> (usize, usize) {
    (3, 64)
}
