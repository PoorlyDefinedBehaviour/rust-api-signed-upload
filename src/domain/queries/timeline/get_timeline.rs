use crate::{
    domain::{
        contracts::{context::Context, deps::Deps},
        value_objects::cursor::Cursor,
    },
    infra::uuid::Uuid,
};
use anyhow::Result;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, PartialOrd)]
pub struct Post {
    pub id: Uuid,
    pub creator_id: Uuid,
    pub description: String,
    pub video_url: String,
    pub likes: i32,
    pub paid: bool,
    pub created_at: DateTime<Utc>,
}

#[tracing::instrument(name = "commands::user::create::run", skip_all, fields(ctx = ?ctx))]
pub async fn handle(deps: &Deps, ctx: &Context, cursor: Cursor) -> Result<Vec<Post>> {
    let posts = deps
        .repos
        .timeline
        .get_timeline(&mut deps.db.read().await?, cursor)
        .await?;

    Ok(posts)
}
