use anyhow::Result;
use crate::domain::contracts::http;
use async_trait::async_trait;
use axum::http::HeaderMap;
use reqwest::{Body, Client, RequestBuilder, Response};

#[derive(Debug)]
pub struct Http {
  client: Client,
}

impl Http {
  pub fn new() -> Self {
    Self {
      client: reqwest::Client::new(),
    }
  }
}

#[async_trait]
impl http::Http for Http {
  async fn get(&self, url: &str, body: Option<Body>, headers: Option<HeaderMap>) -> Result<Response> {
    Ok(build_request(self.client.get(url), body, headers).send().await?)
  }

  async fn post(&self, url: &str, body: Option<Body>, headers: Option<HeaderMap>) -> Result<Response> {
    Ok(build_request(self.client.post(url), body, headers).send().await?)
  }

  async fn put(&self, url: &str, body: Option<Body>, headers: Option<HeaderMap>) -> Result<Response> {
    Ok(build_request(self.client.put(url), body, headers).send().await?)
  }

  async fn delete(&self, url: &str, body: Option<Body>, headers: Option<HeaderMap>) -> Result<Response> {
    Ok(build_request(self.client.delete(url), body, headers).send().await?)
  }
}

fn build_request(mut builder: RequestBuilder, body: Option<Body>, headers: Option<HeaderMap>) -> RequestBuilder {
  if let Some(headers) = headers {
    builder = builder.try_clone().unwrap().headers(headers);
  }

  if let Some(body) = body {
    builder = builder.try_clone().unwrap().body(body);
  }

  builder
}
