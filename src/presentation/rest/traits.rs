//! Contains traits used to extend axum types in order
//! to make it easier to write tests.

use anyhow::{Context, Result};
use async_trait::async_trait;
use hmac::{Hmac, Mac};
use hyper::Body;
use jwt::SignWithKey;
use sha2::Sha256;
use std::collections::HashMap;

use crate::{domain::constants, infra::uuid::Uuid};

#[async_trait]
pub trait RequestBuilderExt {
    /// Generates a JWT token containing the user id and adds it to the request.
    fn with_user_auth(self, user_id: Uuid) -> axum::http::request::Builder;

    fn json<T>(self, value: T) -> Result<hyper::Request<Body>>
    where
        Self: Sized,
        T: serde::Serialize;
}

#[async_trait]
impl RequestBuilderExt for axum::http::request::Builder {
    fn with_user_auth(self, user_id: Uuid) -> axum::http::request::Builder {
        // TODO: hide the jwt library in a module.
        let key: Hmac<Sha256> = Hmac::new_from_slice(b"some-secret").unwrap();

        let mut claims = HashMap::new();
        claims.insert("user_id", user_id.to_string());

        let token_str = claims
            .sign_with_key(&key)
            .expect("signing user auth claims");

        self.header(constants::AUTHORIZATION_HEADER_NAME, token_str)
    }

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
