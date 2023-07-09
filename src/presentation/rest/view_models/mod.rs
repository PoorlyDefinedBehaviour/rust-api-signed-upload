use serde::{Deserialize, Serialize};

pub mod pix_payment;
pub mod register;
pub mod timeline;
pub mod video;

#[derive(Debug, Deserialize, Serialize)]
pub struct ValidationError {
    pub name: String,
    pub message: String,
}
