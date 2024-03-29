use std::{env::VarError, str::FromStr, time::Duration};

use anyhow::{anyhow, Context, Result};

#[derive(Debug)]
pub struct Config {
    pub env: String,
    pub aws: AwsConfig,
    pub s3: S3Config,
    pub database_ro_url: Option<String>,
    pub database_rw_url: Option<String>,
    pub database_max_connections: u32,
}

#[derive(Debug)]
pub struct AwsConfig {
    pub region: String,
    pub local_endpoint: Option<String>,
}

#[derive(Debug)]
pub struct S3Config {
    pub videos_bucket: String,
    pub presigned_url_expires_in_secs: Duration,
}

pub const LOCAL_ENV: &str = "local";

impl Config {
    #[tracing::instrument(name = "Config::from_env", skip_all)]
    pub fn from_env() -> Result<Self> {
        Ok(Self {
            env: env("ENV")?,
            aws: AwsConfig {
                region: env("AWS_REGION")?,
                local_endpoint: opt_env("AWS_LOCAL_ENDPOINT")?,
            },
            s3: S3Config {
                videos_bucket: env("S3_VIDEOS_BUCKET")?,
                presigned_url_expires_in_secs: Duration::from_secs(env(
                    "S3_PRESIGNED_URL_EXPIRES_IN_SECS",
                )?),
            },
            database_ro_url: opt_env("DATABASE_RO_URL")?,
            database_rw_url: opt_env("DATABASE_RW_URL")?,
            database_max_connections: env("DATABASE_MAX_CONNECTIONS")?,
        })
    }

    #[tracing::instrument(name = "Config::is_local_env", skip_all, fields(local))]
    pub fn is_local_env(&self) -> bool {
        let local = self.env.eq_ignore_ascii_case(LOCAL_ENV);
        tracing::Span::current().record("local", local);
        local
    }
}

#[tracing::instrument(name = "config::env", skip_all, fields(key = %key))]
fn env<T: FromStr>(key: &str) -> Result<T>
where
    <T as FromStr>::Err: std::error::Error,
{
    let value = std::env::var(key)
        .with_context(|| format!("unable to find env variable. key={key}"))?
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
