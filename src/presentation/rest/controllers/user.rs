use anyhow::{Result, bail, anyhow};
use axum::{Json, Extension, response::IntoResponse};
use reqwest::StatusCode;
use std::sync::Arc;
use chrono::Utc;
use crate::domain::contracts::deps::Deps;
use crate::domain::commands::user::create::CreateUserInput;
use crate::domain::value_objects::{email::Email, password::Password};

pub async fn register(
    Json(payload): Json<serde_json::Value>,
    Extension(deps): Extension<Arc<Deps>>
) -> impl IntoResponse {
    let input = match into_input(payload) {
        Ok(input) => input,
        Err(error) => return StatusCode::UNPROCESSABLE_ENTITY,
    };

    let mut executor = match deps.db.write().await {
        Ok(executor) => executor,
        Err(_) => return StatusCode::INTERNAL_SERVER_ERROR,
    };

    let result = deps.repos.users.create(&mut executor, input).await;

    match result {
        Ok(_) => StatusCode::CREATED,
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR,
    }
}

fn into_input(payload: serde_json::Value) -> Result<CreateUserInput> {
    let username = match payload.get("username") {
        Some(username) => username.to_string(),
        None => bail!("Missing password input")
    };

    let email = match payload.get("email") {
        Some(email) => {
            let input = email.as_str().ok_or(anyhow!("The given email is invalid"))?;

            Email::try_from(input)?
        },
        None => bail!("Missing email input")
    };

    let password = match payload.get("password") {
        Some(password) => {
            let input = password.as_str().ok_or(anyhow!("The given password is invalid"))?;

            Password::try_from(input)?
        },
        Nome => bail!("Missing password input")
    };

    Ok(CreateUserInput {
        username,
        email,
        password,
        accepted_terms_at: Utc::now(),
    })
}
