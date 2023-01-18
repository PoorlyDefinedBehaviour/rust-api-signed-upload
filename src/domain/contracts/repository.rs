use anyhow::Result;
use async_trait::async_trait;
use sqlx::postgres::{PgQueryResult, PgRow};
use sqlx::query::Query;
use sqlx::{Executor as SqlxExecutor, Pool, Postgres, Transaction};
use std::fmt::Debug;
use std::marker::PhantomData;
use std::sync::Arc;

use crate::domain::commands;
use crate::domain::queries::timeline::get_timeline::Post;
use crate::domain::value_objects::cursor::Cursor;
use crate::infra::uuid::Uuid;

pub struct Executor<'c, S> {
  inner: ExecutorInner<'c>,
  _p: PhantomData<S>,
}

impl<'c, S> Executor<'c, S> {
  pub fn new(inner: ExecutorInner<'c>) -> Self {
    Self {
      inner,
      _p: PhantomData,
    }
  }
}

impl<'c> Executor<'c, Writable> {
  #[allow(unused)]
  pub async fn transaction(self) -> Result<Executor<'c, Transactional>> {
    match self.inner {
      ExecutorInner::Transaction(_) => unreachable!("called transaction() twice, this is a bug"),
      ExecutorInner::Pool(pool) => {
        let tx = pool.begin().await?;
        Ok(Executor::new(ExecutorInner::Transaction(Box::new(tx))))
      }
    }
  }
}

impl<'c> From<Executor<'c, Writable>> for Executor<'c, Readable> {
  fn from(input: Executor<'c, Writable>) -> Self {
    Self::new(input.inner)
  }
}

impl<'c> From<Executor<'c, Transactional>> for Executor<'c, Writable> {
  fn from(input: Executor<'c, Transactional>) -> Self {
    Self::new(input.inner)
  }
}

pub enum Writable {}

pub enum Readable {}

pub enum Transactional {}

pub enum ExecutorInner<'c> {
  Pool(Pool<Postgres>),
  Transaction(Box<Transaction<'c, Postgres>>),
}

#[async_trait]
pub trait SqlxExt<S> {
  async fn fetch_one_ex<'e>(self, executor: &mut Executor<'e, S>) -> Result<PgRow, sqlx::Error>;

  async fn fetch_optional_ex<'e>(
    self,
    executor: &mut Executor<'e, S>,
  ) -> Result<Option<PgRow>, sqlx::Error>;

  async fn execute_ex<'e>(
    self,
    executor: &mut Executor<'e, S>,
  ) -> Result<PgQueryResult, sqlx::Error>;

  async fn fetch_all_ex<'e>(
    self,
    executor: &mut Executor<'e, S>,
  ) -> Result<Vec<PgRow>, sqlx::Error>;
}

#[async_trait]
impl<'q, A, S> SqlxExt<S> for Query<'q, Postgres, A>
where
  S: Send,
  A: sqlx::IntoArguments<'q, Postgres> + 'q,
{
  async fn fetch_one_ex<'e>(self, executor: &mut Executor<'e, S>) -> Result<PgRow, sqlx::Error> {
    match &mut executor.inner {
      ExecutorInner::Pool(pool) => pool.fetch_one(self).await,
      ExecutorInner::Transaction(tx) => tx.fetch_one(self).await,
    }
  }

  async fn fetch_optional_ex<'e>(
    self,
    executor: &mut Executor<'e, S>,
  ) -> Result<Option<PgRow>, sqlx::Error> {
    match &mut executor.inner {
      ExecutorInner::Pool(pool) => pool.fetch_optional(self).await,
      ExecutorInner::Transaction(tx) => tx.fetch_optional(self).await,
    }
  }

  async fn execute_ex<'e>(
    self,
    executor: &mut Executor<'e, S>,
  ) -> Result<PgQueryResult, sqlx::Error> {
    match &mut executor.inner {
      ExecutorInner::Pool(pool) => pool.execute(self).await,
      ExecutorInner::Transaction(tx) => tx.execute(self).await,
    }
  }

  async fn fetch_all_ex<'e>(
    self,
    executor: &mut Executor<'e, S>,
  ) -> Result<Vec<PgRow>, sqlx::Error> {
    match &mut executor.inner {
      ExecutorInner::Pool(pool) => pool.fetch_all(self).await,
      ExecutorInner::Transaction(tx) => tx.fetch_all(self).await,
    }
  }
}

#[async_trait]
pub trait Database: Send + Sync + Debug {
  async fn read<'c>(&self) -> Result<Executor<'c, Readable>>;
  async fn write<'c>(&self) -> Result<Executor<'c, Writable>>;
}

#[derive(Debug)]
pub struct Repository {
  pub users: Arc<dyn UserRepository>,
  pub timeline: Arc<dyn TimelineRepository>,
}

#[cfg_attr(test, mockall::automock)]
#[async_trait]
pub trait UserRepository: Send + Sync + Debug {
  async fn get_by_id<'c>(&self, executor: &mut Executor<'c, Readable>, id: Uuid);

  async fn create<'c>(
    &self,
    executor: &mut Executor<'c, Writable>,
    input: commands::user::create::CreateUserInput,
  ) -> Result<()>;
}

#[async_trait]
pub trait TimelineRepository: Send + Sync + Debug {
    async fn get_timeline<'c>(&self, executor: &mut Executor<'c, Readable>, cursor: Cursor) -> Result<Vec<Post>>;
}
