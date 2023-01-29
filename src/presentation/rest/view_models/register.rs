use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct RegisterInput {
    pub username: String,
    pub email: String,
    // TODO: password should not be printable by default
    pub password: String,
}
