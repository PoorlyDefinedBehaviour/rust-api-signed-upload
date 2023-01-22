use anyhow::Result;
use async_trait::async_trait;
use sqlx::{postgres::PgRow, Row};

use crate::{
    domain::{
        contracts::{
            self,
            repository::{Executor, SqlxExt},
        },
        queries::timeline::get_timeline::Post,
        value_objects::cursor::Cursor,
    },
    infra::uuid::Uuid,
};

#[derive(Debug)]
pub struct TimelineRepository;

#[async_trait]
impl contracts::repository::TimelineRepository for TimelineRepository {
    #[tracing::instrument(name = "TimelineRepository::get_timeline", skip_all, fields(
        cursor = ?cursor
    ))]
    async fn get_timeline<'c>(
        &self,
        executor: &mut Executor<'c>,
        cursor: Cursor,
    ) -> Result<Vec<Post>> {
        let rows = sqlx::query!(
            "SELECT 
                users.username as user_username,
                posts.id as post_id,
                posts.description as post_description,
                posts.video_url as post_video_url,
                posts.likes as post_likes,
                posts.paid as post_paid,
                posts.created_at as post_created_at
            FROM posts 
            INNER JOIN users
            ON users.id = posts.creator_id
            ORDER BY posts.created_at DESC
            OFFSET $1 LIMIT $2;
            ",
            cursor.offset,
            cursor.limit
        )
        .fetch_all_ex(executor)
        .await?;

        let mut posts: Vec<Post> = Vec::with_capacity(rows.len());

        for row in rows {
            posts.push(Post::try_from(row)?);
        }

        Ok(posts)
    }
}

impl TryFrom<PgRow> for Post {
    type Error = anyhow::Error;

    fn try_from(row: PgRow) -> Result<Self, Self::Error> {
        Ok(Self {
            id: {
                let id: sqlx::types::Uuid = row.try_get("post_id")?;

                Uuid::from_slice(id.as_bytes())?
            },
            creator_username: row.try_get("user_username")?,
            description: row.try_get("post_description")?,
            video_url: row.try_get("post_video_url")?,
            likes: row.try_get("post_likes")?,
            paid: row.try_get("post_paid")?,
            created_at: row.try_get("post_created_at")?,
        })
    }
}
