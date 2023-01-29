use std::{collections::HashMap, str::FromStr};

use crate::domain::constants::AUTHORIZATION_HEADER_NAME;
use crate::infra::uuid::Uuid;
use async_trait::async_trait;
use axum::{
    extract::{FromRequest, RequestParts},
    http::StatusCode,
};
use hmac::{Hmac, Mac};
use jwt::VerifyWithKey;
use serde_json::{json, Value};
use sha2::Sha256;

#[derive(Debug)]
pub struct Auth {
    pub user_id: Uuid,
}

pub struct ExtractAuth(pub Auth);

#[async_trait]
impl<B> FromRequest<B> for ExtractAuth
where
    B: Send + 'static,
{
    type Rejection = (StatusCode, axum::Json<Value>);

    async fn from_request(req: &mut RequestParts<B>) -> Result<Self, Self::Rejection> {
        let token = match req.headers().get(AUTHORIZATION_HEADER_NAME) {
            None => {
                return Err((
                    StatusCode::UNAUTHORIZED,
                    axum::Json(json!({
                        "message":
                            format!("missing authorization header: {AUTHORIZATION_HEADER_NAME}")
                    })),
                ))
            }
            Some(v) => match v.to_str() {
                Err(_err) => {
                    return Err((
                        StatusCode::UNAUTHORIZED,
                        axum::Json(json!({
                            "message": format!("header Authorization value is not valid")
                        })),
                    ))
                }
                Ok(v) => v.to_string(),
            },
        };

        let key: Hmac<Sha256> = Hmac::new_from_slice(b"some-secret").unwrap();

        let claims: HashMap<String, String> = match token.verify_with_key(&key) {
            Err(_err) => {
                return Err((
                    StatusCode::UNAUTHORIZED,
                    axum::Json(json!({ "message": format!("jwt token is invalid") })),
                ))
            }
            Ok(v) => v,
        };

        let user_id = match Uuid::from_str(&claims.get("user_id").unwrap()) {
            Err(_err) => {
                return Err((
                    StatusCode::UNAUTHORIZED,
                    axum::Json(json!({
                        "message": format!("header Authorization value is not a valid uuid")
                    })),
                ))
            }
            Ok(v) => v,
        };

        Ok(ExtractAuth(Auth { user_id }))
    }
}
