use anyhow::{Result};
use axum::{Json, Extension, response::IntoResponse};
use fake::faker::internet::en::FreeEmail;
use fake::{Faker, Dummy, Fake};
use hyper::StatusCode;
use rand::Rng;
use serde::{Deserialize, Serialize};
use tracing::error;
use std::sync::Arc;
use chrono::Utc;

use crate::domain::contracts::deps::Deps;
use crate::domain::commands::user::create::CreateUserInput;
use crate::domain::errors::ValidationError;
use crate::domain::{self, commands};
use crate::domain::value_objects::{email::Email, password::Password};
use crate::presentation::rest::extensions::context::ExtractContext;
use crate::presentation::rest::view_models::register::ValidationErrorViewModel;

#[derive(Debug, Deserialize, Serialize)]
pub struct RegisterViewModel {
  username: String,
  email: String,
  password: String
}

impl From<ValidationError> for axum::response::Response {
  fn from(input: ValidationError) -> Self {
    (StatusCode::UNPROCESSABLE_ENTITY, Json(ValidationErrorViewModel{
      name: input.name,
      message:input.message
    })).into_response()
  }
}

#[tracing::instrument(name = "POST /v1/users")]
pub async fn register(
  Json(payload): Json<RegisterViewModel>,
  Extension(deps): Extension<Arc<Deps>>,
  ExtractContext(ctx): ExtractContext,
) -> Result<StatusCode, axum::response::Response> {

  if let Err(error) = commands::user::create::run(&deps, &ctx, payload.try_into()?).await {
    error!(?error, "unable to create user");
    return Err((StatusCode::INTERNAL_SERVER_ERROR, "Internal server error").into_response());
  }

   Ok(StatusCode::CREATED)
}

impl TryFrom<RegisterViewModel> for domain::commands::user::create::CreateUserInput {
  type Error = ValidationError;

  fn try_from(input: RegisterViewModel) -> Result<Self, Self::Error> {
    Ok(CreateUserInput {
      username:input.username,
      email: Email::try_from(input.email)?,
      password:Password::try_from(input.password.as_str())?,
      accepted_terms_at: Utc::now(),
    })
  }
}

impl Dummy<Faker> for RegisterViewModel {
  fn dummy_with_rng<R: Rng + ?Sized>(_: &Faker, _rng: &mut R) -> RegisterViewModel {
      RegisterViewModel {
        username: Faker.fake(),
        email: FreeEmail().fake(),
        password: Faker.fake(),
      }
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use axum::{http::Request, body::Body};
  use tower::ServiceExt;
  use fake::Fake;
  use crate::domain::constants::X_REQUEST_ID_HEADER_NAME;

  use crate::presentation::rest::{router, deps};

  #[tokio::test]
  async fn register() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();

    let app = router();

    let user: RegisterViewModel = Faker.fake();

    let req = Request::builder()
      .method("POST")
      .uri("/v1/users")
      .header("Content-Type", "application/json")
      .header(X_REQUEST_ID_HEADER_NAME, 1)
      .extension(deps()?)
      // serde_json::to_string(&user)?
      // .json(&user)?;
      .body(Body::from(serde_json::to_string(&user)?))?;

    let response = app
      .oneshot(req)
      .await?;

    assert_eq!(response.status(), StatusCode::CREATED);

    Ok(())
  }
}
