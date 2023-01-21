use std::collections::HashMap;

use async_trait::async_trait;
use axum::{
    extract::{FromRequest, RequestParts},
    http::StatusCode,
};
use serde_json::{json, Value};
use crate::domain::contracts::context::Context;
use crate::domain::constants::X_REQUEST_ID_HEADER_NAME;

pub struct ExtractContext(pub Context);


#[async_trait]
impl<B> FromRequest<B> for ExtractContext
where
    B: Send + 'static,
{
    type Rejection = (StatusCode, axum::Json<Value>);

    async fn from_request(req: &mut RequestParts<B>) -> Result<Self, Self::Rejection> {
        let request_id = match req.headers().get(X_REQUEST_ID_HEADER_NAME) {
            None => return Err((StatusCode::BAD_REQUEST, axum::Json(json!({
                "message": format!("missing request id header: {X_REQUEST_ID_HEADER_NAME}")
            })))),
            Some(v) => v.to_str().unwrap().to_owned()
        };

        Ok(ExtractContext(Context::from(HashMap::from([(X_REQUEST_ID_HEADER_NAME.to_owned(), request_id)]))))
    }
}
