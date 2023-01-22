use anyhow::Result;
use async_trait::async_trait;
use sqlx::postgres::{PgQueryResult, PgRow};
use sqlx::query::{Map, Query};
use sqlx::{Executor as SqlxExecutor, Pool, Postgres, Transaction};
use std::fmt::Debug;
use std::sync::Arc;

use crate::domain::commands;
use crate::domain::queries::timeline::get_timeline::Post;
use crate::domain::value_objects::cursor::Cursor;
use crate::infra::uuid::Uuid;

pub struct Executor<'c> {
    inner: ExecutorInner<'c>,
}

impl<'c> Executor<'c> {
    pub fn new(inner: ExecutorInner<'c>) -> Self {
        Self { inner }
    }
}

pub enum ExecutorInner<'c> {
    Pool(Pool<Postgres>),
    Transaction(Box<Transaction<'c, Postgres>>),
}

impl<'c> Executor<'c> {
    #[allow(unused)]
    pub async fn transaction(self) -> Result<Executor<'c>> {
        match self.inner {
            ExecutorInner::Transaction(_) => {
                unreachable!("called transaction() twice, this is a bug")
            }
            ExecutorInner::Pool(pool) => {
                let tx = pool.begin().await?;
                Ok(Executor::new(ExecutorInner::Transaction(Box::new(tx))))
            }
        }
    }
}

#[async_trait]
pub trait SqlxExt {
    async fn fetch_one_ex<'e>(self, executor: &mut Executor<'e>) -> Result<PgRow, sqlx::Error>;

    async fn fetch_optional_ex<'e>(
        self,
        executor: &mut Executor<'e>,
    ) -> Result<Option<PgRow>, sqlx::Error>;

    async fn execute_ex<'e>(
        self,
        executor: &mut Executor<'e>,
    ) -> Result<PgQueryResult, sqlx::Error>;

    async fn fetch_all_ex<'e>(self, executor: &mut Executor<'e>)
        -> Result<Vec<PgRow>, sqlx::Error>;
}

#[async_trait]
impl<'q, A> SqlxExt for Query<'q, Postgres, A>
where
    A: sqlx::IntoArguments<'q, Postgres> + 'q,
{
    async fn fetch_one_ex<'e>(self, executor: &mut Executor<'e>) -> Result<PgRow, sqlx::Error> {
        match &mut executor.inner {
            ExecutorInner::Pool(pool) => pool.fetch_one(self).await,
            ExecutorInner::Transaction(tx) => tx.fetch_one(self).await,
        }
    }

    async fn fetch_optional_ex<'e>(
        self,
        executor: &mut Executor<'e>,
    ) -> Result<Option<PgRow>, sqlx::Error> {
        match &mut executor.inner {
            ExecutorInner::Pool(pool) => pool.fetch_optional(self).await,
            ExecutorInner::Transaction(tx) => tx.fetch_optional(self).await,
        }
    }

    async fn execute_ex<'e>(
        self,
        executor: &mut Executor<'e>,
    ) -> Result<PgQueryResult, sqlx::Error> {
        match &mut executor.inner {
            ExecutorInner::Pool(pool) => pool.execute(self).await,
            ExecutorInner::Transaction(tx) => tx.execute(self).await,
        }
    }

    async fn fetch_all_ex<'e>(
        self,
        executor: &mut Executor<'e>,
    ) -> Result<Vec<PgRow>, sqlx::Error> {
        match &mut executor.inner {
            ExecutorInner::Pool(pool) => pool.fetch_all(self).await,
            ExecutorInner::Transaction(tx) => tx.fetch_all(self).await,
        }
    }
}

#[async_trait]
impl<'q, A, F> SqlxExt for Map<'q, Postgres, F, A>
where
    F: Send + 'q,
    A: sqlx::IntoArguments<'q, Postgres> + 'q,
{
    async fn fetch_one_ex<'e>(self, executor: &mut Executor<'e>) -> Result<PgRow, sqlx::Error> {
        match &mut executor.inner {
            ExecutorInner::Pool(pool) => pool.fetch_one(self).await,
            ExecutorInner::Transaction(tx) => tx.fetch_one(self).await,
        }
    }

    async fn fetch_optional_ex<'e>(
        self,
        executor: &mut Executor<'e>,
    ) -> Result<Option<PgRow>, sqlx::Error> {
        match &mut executor.inner {
            ExecutorInner::Pool(pool) => pool.fetch_optional(self).await,
            ExecutorInner::Transaction(tx) => tx.fetch_optional(self).await,
        }
    }

    async fn execute_ex<'e>(
        self,
        executor: &mut Executor<'e>,
    ) -> Result<PgQueryResult, sqlx::Error> {
        match &mut executor.inner {
            ExecutorInner::Pool(pool) => pool.execute(self).await,
            ExecutorInner::Transaction(tx) => tx.execute(self).await,
        }
    }

    async fn fetch_all_ex<'e>(
        self,
        executor: &mut Executor<'e>,
    ) -> Result<Vec<PgRow>, sqlx::Error> {
        match &mut executor.inner {
            ExecutorInner::Pool(pool) => pool.fetch_all(self).await,
            ExecutorInner::Transaction(tx) => tx.fetch_all(self).await,
        }
    }
}

#[async_trait]
pub trait Database: Send + Sync + Debug {
    async fn read<'c>(&self) -> Result<Executor<'c>>;
    async fn write<'c>(&self) -> Result<Executor<'c>>;
}

#[derive(Debug)]
pub struct Repository {
    pub users: Arc<dyn UserRepository>,
    pub timeline: Arc<dyn TimelineRepository>,
}

#[cfg_attr(test, mockall::automock)]
#[async_trait]
pub trait UserRepository: Send + Sync + Debug {
    async fn get_by_id<'c>(&self, executor: &mut Executor<'c>, id: Uuid);

    async fn create<'c>(
        &self,
        executor: &mut Executor<'c>,
        input: commands::user::CreateUserInput,
    ) -> Result<()>;
}

#[async_trait]
pub trait TimelineRepository: Send + Sync + Debug {
    async fn get_timeline<'c>(
        &self,
        executor: &mut Executor<'c>,
        cursor: Cursor,
    ) -> Result<Vec<Post>>;
}
