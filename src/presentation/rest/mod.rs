mod controllers;
mod extensions;
mod middlewares;
pub mod view_models;

use anyhow::{Context, Result};
use axum::{
    http::header::HeaderName,
    routing::{get, post},
    Extension, Router,
};
use controllers::health_check;
use controllers::pix_payment;
use controllers::timeline;
use controllers::user;
use controllers::video;
use std::sync::Arc;
use tower::ServiceBuilder;
use tower_http::request_id::PropagateRequestIdLayer;
pub mod errors;
pub mod traits;

use crate::{
    config::Config,
    domain::{constants::X_REQUEST_ID_HEADER_NAME, contracts::deps::Deps},
    infra::{self, http::Http},
};

pub async fn router() -> Result<Router> {
    let router = Router::new()
        .route("/v1/health-check", get(health_check::handle))
        .route("/v1/users", post(user::register))
        .route("/v1/timeline", get(timeline::get_timeline))
        .route("/v1/payments/pix", post(pix_payment::start_pix_payment))
        .route("/v1/videos", post(video::start_video_upload))
        .route_layer(ServiceBuilder::new().layer(PropagateRequestIdLayer::new(
            HeaderName::from_static(X_REQUEST_ID_HEADER_NAME),
        )))
        .layer(Extension(Arc::new(
            deps().await.context("instantiating dependencies"),
        )));

    Ok(router)
}

async fn deps() -> Result<Deps> {
    let config = Arc::new(Config::from_env()?);

    let db = infra::repository::Database::new(infra::repository::Config::from(config.as_ref()))?;

    let http = Http::new();

    let s3 = infra::object_storage::s3::S3::new(Arc::clone(&config)).await?;

    Ok(Deps {
        config,
        object_storage: Arc::new(s3),
        db: Arc::new(db),
        http: Arc::new(http),
        repos: infra::repository::new(),
    })
}
