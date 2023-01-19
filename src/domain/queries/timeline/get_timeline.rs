use anyhow::Result;
use serde::{Deserialize, Serialize};
use crate::{domain::{contracts::{deps::Deps, context::Context}, value_objects::cursor::Cursor}, infra::uuid::Uuid};

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, PartialOrd)]
pub struct Post {
    pub id: Uuid,
    pub content_creator_username: String,
    pub content_creator_avatar_url: String,
    pub description: Option<String>,
    pub media_url: String,
    pub likes: i32,
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
