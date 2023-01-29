use crate::domain::{commands, contracts::deps::Deps};
use crate::presentation::rest::errors::error_into_response;
use crate::presentation::rest::extensions::context::ExtractContext;
use crate::presentation::rest::extensions::user::ExtractAuth;
use crate::presentation::rest::view_models;
use axum::{Extension, Json};
use std::sync::Arc;
use tracing::error;

#[tracing::instrument(name = "POST /v1/videos", skip_all, fields(
    ctx = ?ctx
))]
pub async fn start_video_upload(
    ExtractAuth(auth): ExtractAuth,
    Extension(deps): Extension<Arc<Deps>>,
    ExtractContext(ctx): ExtractContext,
) -> Result<Json<view_models::video::StartVideoUploadOutput>, axum::response::Response> {
    let input = commands::video::StartVideoUploadInput {
        user_id: auth.user_id,
    };

    match commands::video::start_video_upload(&deps, &ctx, input).await {
        Ok(output) => Ok(Json(view_models::video::StartVideoUploadOutput::from(
            output,
        ))),
        Err(error) => {
            error!(?error, "unable start video upload");

            Err(error_into_response(error))
        }
    }
}

#[cfg(test)]
mod start_video_upload_tests {
    use hyper::{Body, Method, Request};
    use tokio_util::codec::{BytesCodec, FramedRead};
    use tower::Service;

    use crate::{
        domain::constants::X_REQUEST_ID_HEADER_NAME,
        infra::factory,
        presentation::rest::{
            deps, router,
            traits::{RequestBuilderExt, ResponseExt},
        },
    };

    use reqwest::multipart::{self, Part};

    use super::*;

    #[tokio::test]
    async fn can_get_presigned_post_url_and_upload() -> Result<(), Box<dyn std::error::Error>> {
        dotenv::dotenv().ok();

        let deps = Arc::new(deps().await?);

        let mut executor = deps.db.write().await?;

        let user_id = factory::user::create(&mut executor).await?;

        let mut app = router().await?;

        // Get a presigned url.
        let req = Request::builder()
            .method(Method::POST)
            .uri("/v1/videos")
            .header("Content-Type", "application/json")
            .header(X_REQUEST_ID_HEADER_NAME, 1)
            .with_user_auth(user_id)
            .extension(Arc::clone(&deps))
            .body(Body::empty())?;

        let response = app.call(req).await?;

        assert!(response.status().is_success());

        let start_video_video_upload_response_body: view_models::video::StartVideoUploadOutput =
            response.json().await?;

        assert!(!start_video_video_upload_response_body
            .presigned_url
            .endpoint
            .is_empty());

        let file = include_bytes!("./testdata/image1.png");
        assert!(!file.is_empty());
        let stream = FramedRead::new(file.as_ref(), BytesCodec::new());
        let file_body = Body::wrap_stream(stream);

        // Upload video directly to object storage.
        let mut form = multipart::Form::new();
        for field in start_video_video_upload_response_body
            .presigned_url
            .form_data_fields
            .iter()
        {
            form = form.text(field.name.clone(), field.value.clone());
        }

        form = form.part(
            "file",
            Part::stream(file_body)
                .file_name("image1.png")
                .mime_str("image/png")?,
        );

        let client = reqwest::Client::new();

        let response = client
            .post(
                start_video_video_upload_response_body
                    .presigned_url
                    .endpoint,
            )
            .multipart(form)
            .send()
            .await?;

        assert!(response.status().is_success());

        // Get the video from object storage.
        let value = deps
            .object_storage
            .get(
                &deps.config.s3.videos_bucket,
                &start_video_video_upload_response_body.video_id.to_string(),
            )
            .await?;

        assert_eq!(Some(file.to_vec()), value);

        Ok(())
    }
}
