mod config;
mod domain;
mod infra;
mod presentation;

use anyhow::Result;
use std::net::SocketAddr;
use tracing::info;
use tracing_bunyan_formatter::{BunyanFormattingLayer, JsonStorageLayer};
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::Registry;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv::dotenv().ok();

    let (non_blocking_writer, _guard) = tracing_appender::non_blocking(std::io::stdout());

    let app_name = concat!(env!("CARGO_PKG_NAME"), "-", env!("CARGO_PKG_VERSION")).to_string();

    let bunyan_formatting_layer = BunyanFormattingLayer::new(app_name, non_blocking_writer);

    let subscriber = Registry::default()
        .with(JsonStorageLayer)
        .with(bunyan_formatting_layer);

    tracing::subscriber::set_global_default(subscriber).unwrap();

    let app = presentation::rest::router().await?;

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    info!("server started on {}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();

    Ok(())
}
