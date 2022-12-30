use std::{env::VarError, str::FromStr};

use anyhow::{anyhow, Result};

pub struct Config {
  pub database_ro_url: Option<String>,
  pub database_rw_url: Option<String>,
  pub database_max_connections: u32,
}

impl Config {
  #[tracing::instrument(name = "Config::from_env", skip_all)]
  pub fn from_env() -> Result<Self> {
    Ok(Self {
      database_ro_url: opt_env("DATABASE_RO_URL")?,
      database_rw_url: opt_env("DATABASE_RW_URL")?,
      database_max_connections: env("DATABASE_MAX_CONNECTIONS")?,
    })
  }
}

#[tracing::instrument(name = "config::env", skip_all, fields(key = %key))]
fn env<T: FromStr>(key: &str) -> Result<T>
where
  <T as FromStr>::Err: std::error::Error,
{
  let value = std::env::var(key)?
    .parse()
    .map_err(|err| anyhow!("unable to parse value into expected type error={:?}", err))?;
  Ok(value)
}

#[tracing::instrument(name = "Config::opt_env", skip_all, fields(key = %key))]
fn opt_env<T: FromStr>(key: &str) -> Result<Option<T>>
where
  <T as FromStr>::Err: std::error::Error,
{
  match std::env::var(key) {
    Err(VarError::NotPresent) => Ok(None),
    Err(VarError::NotUnicode(v)) => Err(anyhow!("value is not unicode value={:?}", v)),
    Ok(v) => Ok(Some(v.parse().map_err(|err| {
      anyhow!("unable to parse value into expected type error={:?}", err)
    })?)),
  }
}
