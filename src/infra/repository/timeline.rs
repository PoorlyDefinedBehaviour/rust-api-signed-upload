use async_trait::async_trait;
use anyhow::Result;
use sqlx::{Row, postgres::PgRow};

use crate::{domain::{
    contracts::{
    self,
    repository::{Executor, Readable, SqlxExt}},
    value_objects::cursor::Cursor,
    queries::timeline::get_timeline::Post
    },
    infra::uuid::Uuid
};

#[derive(Debug)]
pub struct TimelineRepository;

#[async_trait]
impl contracts::repository::TimelineRepository for TimelineRepository {
    #[tracing::instrument(name = "TimelineRepository::get_timeline", skip_all, fields(
        cursor = ?cursor
    ))]
    async fn get_timeline<'c>(&self, executor: &mut Executor<'c, Readable>, cursor: Cursor) -> Result<Vec<Post>> {
        let rows = sqlx::query(
                "SELECT * FROM timeline OFFSET $1 LIMIT $2;"
            )
            .bind(cursor.offset)
            .bind(cursor.limit)
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
                let id: sqlx::types::Uuid = row.try_get("id")?;

                Uuid::from_slice(id.as_bytes())?
            },
            content_creator_username: row.try_get("content_creator_username")?,
            content_creator_avatar_url: row.try_get("content_creator_avatar_url")?,
            description: row.try_get("description")?,
            media_url: row.try_get("media_url")?,
            likes: row.try_get("likes")?
        })
    }
}
