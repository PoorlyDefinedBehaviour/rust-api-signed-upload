use std::sync::Arc;

use super::{repository::{Database, Repository}};
use super::http::Http;

pub struct Deps {
  pub db: Arc<dyn Database>,
  pub repos: Repository,
  pub http: Arc<dyn Http>
}