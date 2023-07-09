use std::{str::FromStr, sync::Arc};

use crate::{config::Config, domain};
use anyhow::Result;
use async_trait::async_trait;

use aws_credential_types::provider::ProvideCredentials;
use aws_sdk_s3::Credentials;
use base64::{engine::general_purpose, Engine};
use chrono::{SecondsFormat, Utc};
use hmac::{Hmac, Mac};
use rusoto_s3::{GetObjectRequest, S3Client, S3 as rusotoS3};
use sha2::Sha256;
use tokio::io::AsyncReadExt;
use tracing::info;

pub struct S3 {
    config: Arc<Config>,
    credentials: Credentials,
    rusoto_client: S3Client,
}

impl S3 {
    #[tracing::instrument(name = "S3::new", skip_all, fields(
        config = ?config
    ))]
    pub async fn new(config: Arc<Config>) -> Result<Self> {
        let credentials_provider =
            aws_config::default_provider::credentials::default_provider().await;

        let credentials = credentials_provider.provide_credentials().await?;

        let rusoto_client = S3Client::new(if config.is_local_env() {
            let endpoint = config.aws.local_endpoint.clone().unwrap();
            info!(?endpoint, "setting rusoto S3 endpoint to local endpoint");
            rusoto_core::Region::Custom {
                name: config.aws.region.clone(),
                endpoint: endpoint,
            }
        } else {
            rusoto_core::Region::from_str(&config.aws.region)?
        });

        Ok(Self {
            credentials,
            config,
            rusoto_client,
        })
    }
}

#[async_trait]
impl domain::contracts::object_storage::ObjectStorage for S3 {
    #[tracing::instrument(name = "S3::get_presigned_post_url", skip_all, fields(
        bucket = ?bucket,
        key = ?key,
        url
    ))]
    async fn get_presigned_post_url(
        &self,
        bucket: &str,
        key: &str,
    ) -> Result<domain::contracts::object_storage::GetPresignedPostUrlOutput> {
        let expiration = Utc::now()
            + chrono::Duration::seconds(
                self.config.s3.presigned_url_expires_in_secs.as_secs() as i64
            );

        let expiration_rfc3339 = expiration.to_rfc3339_opts(SecondsFormat::Micros, true);

        let date = Utc::now();
        let date_yyyy_mm_dd = date.format("%Y%m%d").to_string();
        let date_iso_8601_basic_format = date.format("%Y%m%dT%H%M%SZ").to_string();

        let x_amz_credential = format!(
            "{}/{}/{}/s3/aws4_request",
            self.credentials.access_key_id(),
            date_yyyy_mm_dd,
            self.config.aws.region
        );

        let conditions =
            serde_json::to_string(&serde_json::json!({ "expiration": expiration_rfc3339,
              "conditions": [
                {"bucket": bucket},
                ["starts-with", "$key", key],
                // {"success_action_redirect": "http://sigv4examplebucket.s3.amazonaws.com/successful_upload.html"},
                // ["starts-with", "$Content-Type", "image/"],
                // {"x-amz-meta-uuid": "14365123651274"},
                // {"x-amz-server-side-encryption": "AES256"},
                // ["starts-with", "$x-amz-meta-tag", ""],
                {"x-amz-credential": x_amz_credential},
                {"x-amz-algorithm": "AWS4-HMAC-SHA256"},
                {"x-amz-date": date_iso_8601_basic_format},
                ["content-length-range", 1000, 10_485_760]
              ]
            }))?;

        let base64_conditions = general_purpose::STANDARD.encode(&conditions);

        let signature = {
            let mut date_key: Hmac<Sha256> = Hmac::new_from_slice(
                format!(
                    "AWS4{secret_access_key}",
                    secret_access_key = self.credentials.secret_access_key(),
                )
                .as_bytes()
                .as_ref(),
            )?;
            date_key.update(date_yyyy_mm_dd.as_bytes().as_ref());

            let mut date_region_key: Hmac<Sha256> =
                Hmac::new_from_slice(date_key.finalize().into_bytes().as_ref())?;
            date_region_key.update(self.config.aws.region.as_bytes().as_ref());

            let mut date_region_service_key: Hmac<Sha256> =
                Hmac::new_from_slice(date_region_key.finalize().into_bytes().as_ref())?;
            date_region_service_key.update("s3".as_bytes());

            let mut signing_key: Hmac<Sha256> =
                Hmac::new_from_slice(date_region_service_key.finalize().into_bytes().as_ref())?;
            signing_key.update("aws4_request".as_bytes());

            let mut signature: Hmac<Sha256> =
                Hmac::new_from_slice(signing_key.finalize().into_bytes().as_ref())?;
            signature.update(base64_conditions.as_bytes().as_ref());

            hex::encode(signature.finalize().into_bytes())
        };

        let output = domain::contracts::object_storage::GetPresignedPostUrlOutput {
            endpoint: if self.config.is_local_env() {
                format!(
                    "{}/{}",
                    self.config.aws.local_endpoint.clone().unwrap(),
                    self.config.s3.videos_bucket
                )
            } else {
                format!(
                    "http://{bucket}.s3.amazonaws.com",
                    bucket = self.config.s3.videos_bucket,
                )
            },
            form_data_fields: vec![
                domain::contracts::object_storage::FormDataField {
                    name: "key".to_owned(),
                    value: key.to_owned(),
                },
                domain::contracts::object_storage::FormDataField {
                    name: "X-Amz-Credential".to_owned(),
                    value: x_amz_credential,
                },
                domain::contracts::object_storage::FormDataField {
                    name: "X-Amz-Algorithm".to_owned(),
                    value: "AWS4-HMAC-SHA256".to_owned(),
                },
                domain::contracts::object_storage::FormDataField {
                    name: "X-Amz-Date".to_owned(),
                    value: date_iso_8601_basic_format,
                },
                domain::contracts::object_storage::FormDataField {
                    name: "Policy".to_owned(),
                    value: base64_conditions.clone(),
                },
                domain::contracts::object_storage::FormDataField {
                    name: "X-Amz-Signature".to_owned(),
                    value: signature,
                },
            ],
        };

        Ok(output)
    }

    #[tracing::instrument(name = "S3::get", skip_all, fields(
        bucket = ?bucket,
        key = ?key
    ))]
    async fn get(&self, bucket: &str, key: &str) -> Result<Option<Vec<u8>>> {
        let response = self
            .rusoto_client
            .get_object(GetObjectRequest {
                bucket: bucket.to_owned(),
                key: key.to_owned(),
                ..GetObjectRequest::default()
            })
            .await?;

        match response.body {
            None => Ok(None),
            Some(stream) => {
                let mut buffer = vec![];
                stream.into_async_read().read_to_end(&mut buffer).await?;
                Ok(Some(buffer))
            }
        }
    }
}
