use anyhow::Result;
use async_trait::async_trait;

#[derive(Debug)]
pub struct GetPresignedPostUrlOutput {
    /// Endpoint to send the FormData to.
    pub endpoint: String,
    /// List of fields that should be included in the form data sent
    /// to the pre signed endpoint.
    pub form_data_fields: Vec<FormDataField>,
}

#[derive(Debug)]
pub struct FormDataField {
    pub name: String,
    pub value: String,
}

#[async_trait]
pub trait ObjectStorage: Send + Sync {
    /// Generates a url that can be used by the client to
    /// upload an object directly to object storage.
    /// The object must be uploaded through FormData.
    /// https://docs.aws.amazon.com/AmazonS3/latest/API/sigv4-post-example.html
    async fn get_presigned_post_url(
        &self,
        bucket: &str,
        key: &str,
    ) -> Result<GetPresignedPostUrlOutput>;

    /// Fetches a value associated with `key` in the `bucket`.
    async fn get(&self, bucket: &str, key: &str) -> Result<Option<Vec<u8>>>;
}
