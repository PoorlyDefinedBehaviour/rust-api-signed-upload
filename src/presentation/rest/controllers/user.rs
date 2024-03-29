use anyhow::Result;
use axum::{response::IntoResponse, Extension, Json};
use chrono::Utc;
use hyper::StatusCode;
use std::sync::Arc;
use tracing::error;

use crate::domain::commands::user::CreateUserInput;
use crate::domain::contracts::deps::Deps;
use crate::domain::errors::ValidationError;
use crate::domain::value_objects::{email::Email, password::Password};
use crate::domain::{self, commands};
use crate::presentation::rest::errors::error_into_response;
use crate::presentation::rest::extensions::context::ExtractContext;
use crate::presentation::rest::view_models;

impl From<ValidationError> for axum::response::Response {
    fn from(input: ValidationError) -> Self {
        (
            StatusCode::UNPROCESSABLE_ENTITY,
            Json(view_models::ValidationError {
                name: input.name,
                message: input.message,
            }),
        )
            .into_response()
    }
}

#[tracing::instrument(name = "POST /v1/users", skip_all, fields(
    payload = ?payload,
    ctx = ?ctx
))]
pub async fn register(
    Json(payload): Json<view_models::register::RegisterInput>,
    Extension(deps): Extension<Arc<Deps>>,
    ExtractContext(ctx): ExtractContext,
) -> Result<StatusCode, axum::response::Response> {
    if let Err(error) = commands::user::create(&deps, &ctx, payload.try_into()?).await {
        error!(?error, "unable to create user");
        return Err(error_into_response(error));
    }

    Ok(StatusCode::CREATED)
}

impl TryFrom<view_models::register::RegisterInput> for domain::commands::user::CreateUserInput {
    type Error = ValidationError;

    fn try_from(input: view_models::register::RegisterInput) -> Result<Self, Self::Error> {
        Ok(CreateUserInput {
            username: input.username,
            email: Email::try_from(input.email)?,
            password: Password::try_from(input.password.as_str())?,
            accepted_terms_at: Utc::now(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        domain::constants::X_REQUEST_ID_HEADER_NAME, presentation::rest::traits::RequestBuilderExt,
    };
    use axum::http::Request;
    use fake::{faker::internet::en::FreeEmail, Dummy, Fake, Faker};
    use rand::Rng;
    use tower::ServiceExt;

    use crate::presentation::rest::{deps, router};

    impl Dummy<Faker> for view_models::register::RegisterInput {
        fn dummy_with_rng<R: Rng + ?Sized>(
            _: &Faker,
            _rng: &mut R,
        ) -> view_models::register::RegisterInput {
            view_models::register::RegisterInput {
                username: Faker.fake(),
                email: FreeEmail().fake(),
                password: Faker.fake(),
            }
        }
    }

    #[tokio::test]
    async fn register() -> Result<(), Box<dyn std::error::Error>> {
        dotenv::dotenv().ok();

        let app = router().await?;

        let user: view_models::register::RegisterInput = Faker.fake();

        let req = Request::builder()
            .method("POST")
            .uri("/v1/users")
            .header("Content-Type", "application/json")
            .header(X_REQUEST_ID_HEADER_NAME, 1)
            .extension(Arc::new(deps().await?))
            .json(&user)?;

        let response = app.oneshot(req).await?;

        assert_eq!(response.status(), StatusCode::CREATED);

        Ok(())
    }
}
