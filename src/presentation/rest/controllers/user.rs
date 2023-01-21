use anyhow::Result;
use axum::{response::IntoResponse, Extension, Json};
use chrono::Utc;
use fake::faker::internet::en::FreeEmail;
use fake::{Dummy, Fake, Faker};
use hyper::StatusCode;
use rand::Rng;
use std::sync::Arc;
use tracing::error;

use crate::domain::commands::user::create::CreateUserInput;
use crate::domain::contracts::deps::Deps;
use crate::domain::errors::ValidationError;
use crate::domain::value_objects::{email::Email, password::Password};
use crate::domain::{self, commands};
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

#[tracing::instrument(name = "POST /v1/users")]
pub async fn register(
    Json(payload): Json<view_models::register::RegisterInput>,
    Extension(deps): Extension<Arc<Deps>>,
    ExtractContext(ctx): ExtractContext,
) -> Result<StatusCode, axum::response::Response> {
    if let Err(error) = commands::user::create::run(&deps, &ctx, payload.try_into()?).await {
        error!(?error, "unable to create user");
        return Err((StatusCode::INTERNAL_SERVER_ERROR, "Internal server error").into_response());
    }

    Ok(StatusCode::CREATED)
}

impl TryFrom<view_models::register::RegisterInput>
    for domain::commands::user::create::CreateUserInput
{
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::constants::X_REQUEST_ID_HEADER_NAME;
    use axum::{body::Body, http::Request};
    use fake::Fake;
    use tower::ServiceExt;

    use crate::presentation::rest::{deps, router};

    #[tokio::test]
    async fn register() -> Result<(), Box<dyn std::error::Error>> {
        dotenv::dotenv().ok();

        let app = router();

        let user: view_models::register::RegisterInput = Faker.fake();

        let req = Request::builder()
            .method("POST")
            .uri("/v1/users")
            .header("Content-Type", "application/json")
            .header(X_REQUEST_ID_HEADER_NAME, 1)
            .extension(deps()?)
            // serde_json::to_string(&user)?
            // .json(&user)?;
            .body(Body::from(serde_json::to_string(&user)?))?;

        let response = app.oneshot(req).await?;

        assert_eq!(response.status(), StatusCode::CREATED);

        Ok(())
    }
}
