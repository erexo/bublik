use anyhow::{Context, Result};
use poem::{listener::TcpListener, Server};
use tracing::{metadata::LevelFilter, trace};

mod api;
mod db;
mod models;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_max_level(LevelFilter::TRACE)
        .with_target(false)
        .init();
    trace!("Hi!");

    let db = db::prepare_database().await?;
    Server::new(TcpListener::bind("51.75.55.235:3710"))
        .run(api::routes(&db))
        .await
        .context("server")
}
