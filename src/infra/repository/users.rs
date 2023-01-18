use crate::infra::uuid::Uuid;
use crate::domain::{
  commands,
  contracts::{
    self,
    repository::{Executor, Readable, SqlxExt, Writable},
  },
};
use anyhow::Result;
use async_trait::async_trait;
use chrono::Utc;

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
    sqlx::query("INSERT INTO users(
        id,
        username,
        email,
        password,
        accepted_terms_at,
        created_at,
        updated_at
      ) VALUES (
        uuid_generate_v4(),
        $1, $2, $3,
        TO_TIMESTAMP($4),
        TO_TIMESTAMP($5),
        TO_TIMESTAMP($6)
      )")
      .bind(input.username)                      // $2->username
      .bind(input.email.expose())                // $3->email
      .bind(input.password.expose())             // $4->password
      .bind(input.accepted_terms_at.timestamp()) // $5->accepted_terms_at
      .bind(Utc::now().timestamp())              // $6->created_at
      .bind(Utc::now().timestamp())              // $7->updated_at
      .execute_ex(executor)
      .await?;
    Ok(())
  }
}
