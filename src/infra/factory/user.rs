use crate::domain::contracts::repository::{Executor, SqlxExt};
use crate::infra::uuid::Uuid;
use anyhow::Result;
use fake::faker::internet::en::FreeEmail;
use fake::{Fake, Faker};

#[allow(dead_code)]
pub async fn create<'c>(executor: &mut Executor<'c>) -> Result<Uuid> {
    let id = Uuid::new_v4();

    sqlx::query!(
        "INSERT INTO users (
            id,
            username,
            email,
            password,
            accepted_terms_at
        ) VALUES (
            $1, $2, $3, $4, CURRENT_TIMESTAMP
        )
        ",
        &id,
        &Faker.fake::<String>(),
        &FreeEmail().fake::<String>(),
        &Faker.fake::<String>(),
    )
    .execute_ex(executor)
    .await?;

    Ok(id)
}

#[allow(dead_code)]
pub async fn refresh<'c>(executor: &mut Executor<'c>) -> Result<()> {
    sqlx::query("DELETE FROM timeline")
        .execute_ex(executor)
        .await?;

    Ok(())
}
