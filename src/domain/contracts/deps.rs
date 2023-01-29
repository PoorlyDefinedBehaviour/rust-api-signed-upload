use std::sync::Arc;

use crate::config::Config;

use super::http::Http;
use super::{
    object_storage::ObjectStorage,
    repository::{Database, Repository},
};

pub struct Deps {
    pub config: Arc<Config>,
    pub db: Arc<dyn Database>,
    pub repos: Repository,
    pub http: Arc<dyn Http>,
    pub object_storage: Arc<dyn ObjectStorage>,
}
