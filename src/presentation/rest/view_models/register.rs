use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct RegisterInput {
    pub username: String,
    pub email: String,
    pub password: String,
}
