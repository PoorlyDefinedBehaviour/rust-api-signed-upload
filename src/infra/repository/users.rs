use crate::domain::{
  commands,
  contracts::{
    self,
    repository::{Executor, Readable, SqlxExt, Writable},
  },
};
use anyhow::Result;
use async_trait::async_trait;
use uuid::Uuid;

#[derive(Debug)]
pub struct UserRepository;

#[async_trait]
impl contracts::repository::UserRepository for UserRepository {
  #[tracing::instrument(name = "UserRepository.get_by_id", skip_all, fields(id = %id))]
  async fn get_by_id<'c>(&self, executor: &mut Executor<'c, Readable>, id: Uuid) {
    let _row = sqlx::query("select * from users where id = $1")
      .bind(id.to_string())
      .fetch_optional_ex(executor)
      .await;
  }

  async fn create<'c>(
    &self,
    executor: &mut Executor<'c, Writable>,
    input: commands::user::create::CreateUserInput,
  ) -> Result<()> {
    sqlx::query("insert into users(username, email, password, accepted_terms_at) values ($1, $2, $3, $4")
      .bind(input.username)
      .bind(input.email.expose())
      .bind(input.password.expose())
      .bind(input.accepted_terms_at.to_rfc3339())
      .execute_ex(executor)
      .await?;
    Ok(())
  }
}
