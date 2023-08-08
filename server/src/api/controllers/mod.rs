pub mod bank;
pub mod card;
pub mod user;
pub mod validation;

mod prelude {
    pub use crate::api::validation_error::ValidationError::*;
    pub use poem::Result;
}

#[derive(poem_openapi::Tags)]
pub enum Tags {
    Validation,
    User,
    Card,
    Bank,
}
