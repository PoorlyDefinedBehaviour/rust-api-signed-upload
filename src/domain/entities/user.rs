use std::hash::Hash;

use chrono::{DateTime, Utc};

use crate::infra::uuid::Uuid;

pub struct User {
    id: Uuid,
    username: String,
    birthdate: DateTime<Utc>,
    email: String,
    password: String,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
    banned_at: DateTime<Utc>,
    deleted_at: DateTime<Utc>,
}
