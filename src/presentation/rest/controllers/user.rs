use anyhow::{Result, bail, anyhow, Context};
use axum::{Json, Extension, response::IntoResponse};
use reqwest::StatusCode;
use std::sync::Arc;
use chrono::Utc;
use crate::domain::contracts::deps::Deps;
use crate::domain::commands::user::create::CreateUserInput;
use crate::domain::value_objects::{email::Email, password::Password};

#[tracing::instrument(name = "POST /v1/users")]
pub async fn register(
    Json(payload): Json<serde_json::Value>,
    Extension(deps): Extension<Arc<Deps>>
) -> impl IntoResponse {
    let input = match into_input(payload) {
        Ok(input) => input,
        Err(error) => return (StatusCode::UNPROCESSABLE_ENTITY, error.to_string()),
    };

    let mut executor = match deps.db.write().await {
        Ok(executor) => executor,
        Err(error) => return (StatusCode::INTERNAL_SERVER_ERROR, error.to_string()),
    };

    let result = deps.repos.users.create(&mut executor, input).await;

    match result {
        Ok(_) => (StatusCode::CREATED, String::from("User successfully registered")),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, String::from("Failed to create user")),
    }
}

fn into_input(payload: serde_json::Value) -> Result<CreateUserInput> {
    let username = match payload.get("username") {
        Some(username) => username.to_string(),
        None => bail!("Missing password input")
    };

    let email = match payload.get("email") {
        Some(email) => {
            let input = email.as_str().context("The given email is invalid")?;

            Email::try_from(input)?
        },
        None => bail!("Missing email input")
    };

    let password = match payload.get("password") {
        Some(password) => {
            let input = password.as_str().context("The given password is invalid")?;

            Password::try_from(input)?
        },
        None => bail!("Missing password input")
    };

    Ok(CreateUserInput {
        username,
        email,
        password,
        accepted_terms_at: Utc::now(),
    })
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use async_trait::async_trait;
    use axum::{http::Request, body::Body};
    use reqwest::StatusCode;
    use serde_json::json;
    use tower::ServiceExt;

    use crate::infra::uuid::Uuid;
    use crate::domain::{
        contracts::repository::{Repository, UserRepository, Executor, Readable, Writable},
        commands::user::create::CreateUserInput
    };
    use crate::presentation::rest::{router, deps};

    #[tokio::test]
    async fn register() -> Result<(), Box<dyn std::error::Error>> {
        let app = router();

        let mut deps = deps()?;

        deps.repos = Repository {
            users: Arc::new(URepository)
        };

        let req = Request::builder()
            .method("POST")
            .uri("/v1/users")
            .extension(deps)
            .body(Body::from(json!({
                "username": "zezinho_handjobber",
                "email": "jose.almeida@punhetinha.com.br",
                "password": "i'm_afraid*of)woman",
            }).to_string()))?;

        let response = app
            .oneshot(req)
            .await?;

        assert_eq!(response.status(), StatusCode::CREATED);

        Ok(())
    }

    #[derive(Debug)]
    struct URepository;

    #[async_trait]
    impl UserRepository for URepository {
        async fn create<'c>(
            &self,
            _executor: &mut Executor<'c, Writable>,
            _input: CreateUserInput
        ) -> anyhow::Result<()> {
            Ok(())
        }

        async fn get_by_id<'c>(&self, _executor: &mut Executor<'c, Readable>, _id: Uuid) {
            panic!("this method shouldn't not be called")
        }
    }

}
