use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct ValidationErrorViewModel {
  pub name: String,
  pub message: String,
}
