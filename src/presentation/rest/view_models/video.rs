use serde::{Deserialize, Serialize};

use crate::{domain::commands, infra::uuid::Uuid};

#[derive(Debug, Deserialize, Serialize)]
pub struct StartVideoUploadOutput {
    pub presigned_url: PresignedPostUrlOutput,
    pub video_id: Uuid,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct PresignedPostUrlOutput {
    /// Endpoint to send the FormData to.
    pub endpoint: String,
    /// List of fields that should be included in the form data sent
    /// to the pre signed endpoint.
    pub form_data_fields: Vec<FormDataField>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct FormDataField {
    pub name: String,
    pub value: String,
}

impl From<commands::video::StartVideoUploadOutput> for StartVideoUploadOutput {
    fn from(input: commands::video::StartVideoUploadOutput) -> Self {
        Self {
            presigned_url: PresignedPostUrlOutput {
                endpoint: input.presigned_url.endpoint,
                form_data_fields: input
                    .presigned_url
                    .form_data_fields
                    .into_iter()
                    .map(|field| FormDataField {
                        name: field.name,
                        value: field.value,
                    })
                    .collect(),
            },
            video_id: input.video_id,
        }
    }
}
