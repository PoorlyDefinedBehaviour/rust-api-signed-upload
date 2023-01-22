//! Contains traits used to extend axum types in order
//! to make it easier to write tests.

use anyhow::{Context, Result};
use async_trait::async_trait;
use hyper::Body;

#[async_trait]
pub trait RequestBuilderExt {
    fn json<T>(self, value: T) -> Result<hyper::Request<Body>>
    where
        Self: Sized,
        T: serde::Serialize;
}

#[async_trait]
impl RequestBuilderExt for axum::http::request::Builder {
    fn json<T>(self, value: T) -> Result<hyper::Request<Body>>
    where
        Self: Sized,
        T: serde::Serialize,
    {
        let s = self
            .body(Body::from(
                serde_json::to_string(&value).context("serializing request body")?,
            ))
            .context("setting request body")?;

        Ok(s)
    }
}

#[async_trait]
pub trait ResponseExt {
    async fn json<T>(self) -> Result<T>
    where
        T: serde::de::DeserializeOwned;
}

#[async_trait]
impl ResponseExt for axum::response::Response {
    async fn json<T>(self) -> Result<T>
    where
        T: serde::de::DeserializeOwned,
    {
        let t: T = serde_json::from_slice(
            &hyper::body::to_bytes(self.into_body())
                .await
                .context("reading response body bytes")?,
        )
        .context("deserializing response body")?;

        Ok(t)
    }
}
