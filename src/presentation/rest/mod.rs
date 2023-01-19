mod controllers;
mod middlewares;
mod extensions;
pub mod view_models;

use std::sync::Arc;
use anyhow::Result;
use axum::{http::header::HeaderName, routing::{get, post}, Extension, Router};
use controllers::health_check;
use tower::ServiceBuilder;
use tower_http::request_id::PropagateRequestIdLayer;
use controllers::user;
use controllers::timeline;

use crate::{
  config::Config,
  domain::{constants::X_REQUEST_ID_HEADER_NAME, contracts::deps::Deps},
  infra::{self, http::Http},
};

pub fn router() -> Router {
  Router::new()
    .route("/v1/health-check", get(health_check::handle))
    .route("/v1/users", post(user::register))
    .route("/v1/timeline", get(timeline::get_timeline))
    .route_layer(ServiceBuilder::new().layer(PropagateRequestIdLayer::new(
      HeaderName::from_static(X_REQUEST_ID_HEADER_NAME),
    )))
    .layer(Extension(Arc::new(deps().unwrap())))
}

fn deps() -> Result<Deps> {
  let config = Config::from_env()?;

  let db = infra::repository::Database::new(infra::repository::Config::from(&config))?;

  let http = Http::new();

  Ok(Deps {
    db: Arc::new(db),
    http: Arc::new(http),
    repos: infra::repository::new(),
  })
}
