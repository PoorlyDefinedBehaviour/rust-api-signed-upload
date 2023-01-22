use serde::{Deserialize, Serialize};

pub mod pix_payment;
pub mod register;

#[derive(Debug, Deserialize, Serialize)]
pub struct ValidationError {
    pub name: String,
    pub message: String,
}
