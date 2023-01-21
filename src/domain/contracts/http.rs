use std::fmt::Debug;

use anyhow::Result;
use async_trait::async_trait;
use axum::http::HeaderMap;
use reqwest::{Response, Body};

#[async_trait]
pub trait Http: Send + Sync + Debug {
  async fn get(
    &self,
    url: &str,
    body: Option<Body>,
    headers: Option<HeaderMap>,
  ) -> Result<Response>;

  async fn post(
    &self,
    url: &str,
    body: Option<Body>,
    headers: Option<HeaderMap>,
  ) -> Result<Response>;

  async fn put(
    &self,
    url: &str,
    body: Option<Body>,
    headers: Option<HeaderMap>,
  ) -> Result<Response>;

  async fn delete(
    &self,
    url: &str,
    body: Option<Body>,
    headers: Option<HeaderMap>,
  ) -> Result<Response>;
}
