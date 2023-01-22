use crate::domain::{
    commands,
    contracts::{
        self,
        repository::{Executor, SqlxExt},
    },
};
use crate::infra::uuid::Uuid;
use anyhow::Result;
use async_trait::async_trait;
use chrono::Utc;

#[derive(Debug)]
pub struct UserRepository;

#[async_trait]
impl contracts::repository::UserRepository for UserRepository {
    #[tracing::instrument(name = "UserRepository.get_by_id", skip_all, fields(id = %id))]
    async fn get_by_id<'c>(&self, executor: &mut Executor<'c>, id: Uuid) {
        let _row = sqlx::query!("select * from users where id = $1", &id)
            .fetch_optional_ex(executor)
            .await;
    }

    async fn create<'c>(
        &self,
        executor: &mut Executor<'c>,
        input: commands::user::CreateUserInput,
    ) -> Result<()> {
        sqlx::query!(
            "INSERT INTO users(
        id,
        username,
        email,
        password,
        accepted_terms_at,
        created_at
      ) VALUES (
        uuid_generate_v4(),
        $1, $2, $3,
        TO_TIMESTAMP($4),
        TO_TIMESTAMP($5)
      )",
            input.username,                             // $2->username
            input.email.expose(),                       // $3->email
            input.password.expose(),                    // $4->password
            input.accepted_terms_at.timestamp() as f64, // $5->accepted_terms_at
            Utc::now().timestamp() as f64,              // $6->created_at
        )
        .execute_ex(executor)
        .await?;
        Ok(())
    }
}
