use async_trait::async_trait;
use anyhow::Result;

use crate::domain::{contracts::{self, repository::{Executor, Readable}}, value_objects::cursor::Cursor, queries::timeline::get_timeline::Post};

#[derive(Debug)]
pub struct TimelineRepository;

#[async_trait]
impl contracts::repository::TimelineRepository for TimelineRepository {
    async fn get_timeline<'c>(&self, executor: &mut Executor<'c, Readable>, cursor: Cursor) -> Result<Vec<Post>> {
        sqlx::query_as(
                "SELECT * FROM timeline OFFSET $1 LIMIT $2;"
            )
            .bind(cursor.offset)
            .bind(cursor.limit)
            .fetch_all_ex(executor)
            .await?
    }
}
