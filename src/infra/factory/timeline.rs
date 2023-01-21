use crate::domain::contracts::repository::{Executor, SqlxExt, Writable};
use crate::domain::queries::timeline::get_timeline::Post;
use crate::infra::uuid::Uuid;
use anyhow::Result;
use fake::{Dummy, Fake, Faker};

impl Dummy<Faker> for Post {
    fn dummy_with_rng<R: rand::Rng + ?Sized>(_config: &Faker, _rng: &mut R) -> Self {
        Self {
            id: Uuid::new_v4(),
            content_creator_username: Faker.fake(),
            content_creator_avatar_url: Faker.fake(),
            media_url: Faker.fake(),
            description: Faker.fake(),
            likes: Faker.fake(),
        }
    }
}

#[allow(dead_code)]
pub async fn create_with<'c>(post: &Post, executor: &mut Executor<'c, Writable>) -> Result<()> {
    sqlx::query(
        "INSERT INTO timeline(
            id,
            content_creator_username,
            content_creator_avatar_url,
            description,
            media_url,
            likes
        ) VALUES (
            uuid_generate_v4(),
            $1, $2, $3, $4, $5
        )",
    )
    .bind(&post.content_creator_username)
    .bind(&post.content_creator_avatar_url)
    .bind(&post.description)
    .bind(&post.media_url)
    .bind(&post.likes)
    .execute_ex(executor)
    .await?;

    Ok(())
}

#[allow(dead_code)]
pub async fn create_many<'c>(
    count: usize,
    executor: &mut Executor<'c, Writable>,
) -> Result<Vec<Post>> {
    let mut posts = Vec::with_capacity(count);

    for _ in 0..count {
        let post = Faker.fake();

        create_with(&post, executor).await?;
        posts.push(post)
    }

    Ok(posts)
}

#[allow(dead_code)]
pub async fn refresh<'c>(executor: &mut Executor<'c, Writable>) -> Result<()> {
    sqlx::query("DELETE FROM timeline")
        .execute_ex(executor)
        .await?;

    Ok(())
}
