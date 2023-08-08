use poem::{
    http::StatusCode, middleware::CatchPanic, Endpoint, EndpointExt, IntoEndpoint, Middleware,
    Route,
};
use poem_openapi::OpenApiService;
use sqlx::{Pool, Sqlite};
use tracing::error;

pub mod controllers;
pub mod trace_error;
pub mod validation_error;

pub fn routes(db: &Pool<Sqlite>) -> impl IntoEndpoint {
    use controllers::*;
    let controllers = (validation::Api, user::api(db), card::api(db), bank::api(db));
    let api = OpenApiService::new(controllers, "Klaudia", "1.0");
    let docs = api.swagger_ui();
    Route::new()
        .nest("/", api)
        .nest("/swagger", docs)
        .with(catch_panic())
        .with(trace_error::TraceError)
}

fn catch_panic<E: Endpoint>() -> impl Middleware<E> {
    CatchPanic::new().with_handler(|err| {
        error!("{:?}", err);
        StatusCode::INTERNAL_SERVER_ERROR
    })
}
