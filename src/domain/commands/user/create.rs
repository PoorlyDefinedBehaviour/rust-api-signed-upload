use crate::domain::contracts::{context::Context, deps::Deps};
use crate::domain::value_objects::{email::Email, password::Password};
use anyhow::Result;
use chrono::{DateTime, Utc};

pub struct CreateUserInput {
    pub username: String,
    pub email: Email,
    pub password: Password,
    pub accepted_terms_at: DateTime<Utc>,
}

#[tracing::instrument(name = "commands::user::create", skip_all, fields(ctx = ?ctx))]
pub async fn create(deps: &Deps, ctx: &Context, input: CreateUserInput) -> Result<()> {
    deps.repos
        .users
        .create(&mut deps.db.write().await?, input)
        .await?;

    Ok(())
}
