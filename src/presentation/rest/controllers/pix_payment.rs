use axum::{Extension, Json};
use std::sync::Arc;
use tracing::error;

use crate::domain::commands;
use crate::domain::contracts::deps::Deps;
use crate::presentation::rest::errors::error_into_response;
use crate::presentation::rest::extensions::context::ExtractContext;
use crate::presentation::rest::view_models;
use crate::presentation::rest::view_models::pix_payment::StartPixPaymentInput;

#[tracing::instrument(name = "POST /v1/payments/pix", skip_all, fields(
    payload = ?payload,
    ctx = ?ctx
))]
pub async fn start_pix_payment(
    Json(payload): Json<StartPixPaymentInput>,
    Extension(deps): Extension<Arc<Deps>>,
    ExtractContext(ctx): ExtractContext,
) -> Result<Json<view_models::pix_payment::StartPixPaymentOutput>, axum::response::Response> {
    match commands::pix_payment::start_payment(&deps, &ctx, payload.into()).await {
        Ok(output) => Ok(Json(output.into())),
        Err(error) => {
            error!(?error, "unable to start pix payment");
            dbg!(&error);
            Err(error_into_response(error))
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::domain::constants::X_REQUEST_ID_HEADER_NAME;
    use crate::infra::uuid::Uuid;
    use crate::presentation::rest::traits::{RequestBuilderExt, ResponseExt};
    use crate::presentation::rest::{router, view_models};
    use axum::http::Request;
    use hyper::Method;
    use tower::ServiceExt;

    #[tokio::test]
    async fn can_start_pix_payment() -> Result<(), Box<dyn std::error::Error>> {
        dotenv::dotenv().ok();

        let app = router();

        let body = view_models::pix_payment::StartPixPaymentInput {
            creator_id: Uuid::new_v4(),
            subscription: view_models::pix_payment::SubscriptionInput::Monthly,
        };

        let req = Request::builder()
            .method(Method::POST)
            .uri("/v1/payments/pix")
            .header("Content-Type", "application/json")
            .header(X_REQUEST_ID_HEADER_NAME, 1)
            .json(&body)?;

        let response = app.oneshot(req).await?;

        assert!(response.status().is_success());

        let response_body: view_models::pix_payment::StartPixPaymentOutput =
            response.json().await?;

        assert!(!response_body.qrcode.is_empty());

        Ok(())
    }
}
