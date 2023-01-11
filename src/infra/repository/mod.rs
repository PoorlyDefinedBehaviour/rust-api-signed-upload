use crate::{
  config,
  domain::contracts::{
    self,
    repository::{Executor, ExecutorInner, Readable, Writable},
  },
};
use anyhow::{Context, Result};
use async_trait::async_trait;
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};
use tokio::sync::RwLock;

pub mod users;

#[derive(Debug)]
pub struct Config {
  database_ro_url: Option<String>,
  database_rw_url: Option<String>,
  max_connections: u32,
}

impl From<&config::Config> for Config {
  fn from(input: &config::Config) -> Self {
    Self {
      database_ro_url: input.database_ro_url.clone(),
      database_rw_url: input.database_rw_url.clone(),
      max_connections: input.database_max_connections,
    }
  }
}

#[derive(Debug)]
pub struct Database {
  config: Config,
  ro_pool: RwLock<Option<Pool<Postgres>>>,
  #[allow(unused)]
  rw_pool: RwLock<Option<Pool<Postgres>>>,
}

impl Database {
  pub fn new(config: Config) -> Result<Self> {
    Ok(Self {
      config,
      ro_pool: RwLock::new(None),
      rw_pool: RwLock::new(None),
    })
  }

  async fn ro_pool(&self) -> Result<Pool<Postgres>> {
    let url = self
      .config
      .database_ro_url
      .as_ref()
      .context("missing database_ro_url")?;

    let mut pool = self.ro_pool.write().await;
    match &*pool {
      None => {
        let new_pool = PgPoolOptions::new()
          .max_connections(self.config.max_connections)
          .connect(url)
          .await
          .context("failed to connect to database_ro_url")?;

        *pool = Some(new_pool.clone());
        Ok(new_pool)
      }
      Some(pool) => Ok(pool.clone()),
    }
  }

  async fn rw_pool(&self) -> Result<Pool<Postgres>> {
    let url = self
      .config
      .database_rw_url
      .as_ref()
      .context("missing database_rw_url")?;

    let mut pool = self.ro_pool.write().await;
    match &*pool {
      None => {
        let new_pool = PgPoolOptions::new()
          .max_connections(self.config.max_connections)
          .connect(url)
          .await
          .context("failed to connect to database_rw_url")?;

        *pool = Some(new_pool.clone());
        Ok(new_pool)
      }
      Some(pool) => Ok(pool.clone()),
    }
  }
}

#[async_trait]
impl contracts::repository::Database for Database {
  async fn read<'c>(&self) -> Result<Executor<'c, Readable>> {
    Ok(Executor::new(ExecutorInner::Pool(self.ro_pool().await?)))
  }

  async fn write<'c>(&self) -> Result<Executor<'c, Writable>> {
    Ok(Executor::new(ExecutorInner::Pool(self.rw_pool().await?)))
  }
}

pub fn new() -> contracts::repository::Repository {
  todo!()
}
