use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::{domain::queries, infra::uuid::Uuid};

#[derive(Debug, Deserialize, Serialize)]
pub struct PostOutput {
    pub id: Uuid,
    pub creator_username: String,
    pub description: String,
    pub video_url: String,
    pub likes: i32,
    pub paid: bool,
    pub created_at: DateTime<Utc>,
}

impl From<queries::timeline::get_timeline::Post> for PostOutput {
    fn from(input: queries::timeline::get_timeline::Post) -> Self {
        Self {
            id: input.id,
            creator_username: input.creator_username,
            description: input.description,
            video_url: input.video_url,
            likes: input.likes,
            paid: input.paid,
            created_at: input.created_at,
        }
    }
}
