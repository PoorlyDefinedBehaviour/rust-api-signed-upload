use crate::domain::{
  contracts::{context::Context, deps::Deps},
  value_objects::email::Email,
};
use anyhow::Result;

pub struct CreateUserInput {
  pub name: String,
  pub email: Email,
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
