use crate::domain::contracts::{context::Context, deps::Deps};
use crate::domain::value_objects::{
    password::Password,
    email::Email
};
use anyhow::Result;
use chrono::{DateTime, Utc};

pub struct CreateUserInput {
  pub username: String,
  pub email: Email,
  pub password: Password,
  pub accepted_terms_at: DateTime<Utc>
}

#[tracing::instrument(name = "commands::user::create::run", skip_all, fields(ctx = ?ctx))]
#[allow(dead_code)]
pub async fn run(deps: &Deps, ctx: &Context, input: CreateUserInput) -> Result<()> {
  deps
    .repos
    .users
    .create(&mut deps.db.write().await?, input)
    .await?;

  Ok(())
}
