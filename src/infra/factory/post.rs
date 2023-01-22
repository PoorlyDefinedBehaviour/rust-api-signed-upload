use crate::domain::contracts::repository::{Executor, SqlxExt};
use crate::domain::queries::timeline::get_timeline::Post;
use crate::infra::uuid::Uuid;
use anyhow::Result;
use chrono::Utc;
use fake::{Dummy, Fake, Faker};

impl Dummy<Faker> for Post {
    fn dummy_with_rng<R: rand::Rng + ?Sized>(_config: &Faker, _rng: &mut R) -> Self {
        Self {
            id: Uuid::new_v4(),
            description: Faker.fake(),
            likes: Faker.fake(),
            creator_id: Uuid::new_v4(),
            video_url: Faker.fake(),
            paid: Faker.fake(),
            created_at: Utc::now(),
        }
    }
}

#[allow(dead_code)]
pub async fn create_for_user<'c>(user_id: Uuid, executor: &mut Executor<'c>) -> Result<()> {
    sqlx::query!(
        "INSERT INTO posts (
            id,
            creator_id,
            description,
            video_url,
            likes,
            paid
        ) VALUES (
            $1, $2, $3, $4, $5, $6
        )",
        &Uuid::new_v4(),
        &user_id,
        &Faker.fake::<String>(),
        &Faker.fake::<String>(),
        &Faker.fake::<i32>(),
        &Faker.fake::<bool>(),
    )
    .execute_ex(executor)
    .await?;

    Ok(())
}

#[allow(dead_code)]
pub async fn refresh<'c>(executor: &mut Executor<'c>) -> Result<()> {
    sqlx::query("DELETE FROM timeline")
        .execute_ex(executor)
        .await?;

    Ok(())
}
