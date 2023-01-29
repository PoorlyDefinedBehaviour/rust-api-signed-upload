
use crate::{
    domain::{contracts::{context::Context, deps::Deps}, self},
    infra::uuid::Uuid,
};
use anyhow::Result;
use tracing::info;

#[derive(Debug)]
pub struct StartVideoUploadInput {
    /// Id of the user that wants to upload a video.
    pub user_id: Uuid,
}

#[derive(Debug)]
pub struct StartVideoUploadOutput {
    /// Contains endpoint that can be used to upload a video from the client
    /// direct to our object storage without passing through our servers.
    pub presigned_url: domain::contracts::object_storage::GetPresignedPostUrlOutput,

    /// Id identifiying the video in the object storage.
    pub video_id: Uuid
}

#[derive(Debug, thiserror::Error)]
pub enum UploadVideoError {
    #[error("user is not allowed to upload videos")]
    UserNotAllowedToUploadVideos,
}

#[tracing::instrument(name = "commands::video::start_video_upload", skip_all, fields(
    ctx = ?ctx,
    input = ?input,
))]
pub async fn start_video_upload(
    deps: &Deps,
    ctx: &Context,
    input: StartVideoUploadInput,
) -> Result<StartVideoUploadOutput> {
    if !is_user_allowed_to_upload_videos(deps, input.user_id) {
        info!("user is not allowed to upload videos");
        return Err(UploadVideoError::UserNotAllowedToUploadVideos.into());
    }

    let video_id = Uuid::new_v4();

    let presigned_url = 
        deps.object_storage.get_presigned_post_url(&deps.config.s3.videos_bucket, &video_id.to_string()).await?;

    Ok(StartVideoUploadOutput {
        video_id,
        presigned_url,
    })
}

// TODO: check if user can upload a video. Is the user a content creator?
// Users that just view videos from content creators should not be able to upload
// videos.
#[tracing::instrument(name = "commands::video::is_user_allowed_to_upload_videos", skip_all, fields(
    user_id = ?user_id,
    allowed 
))]
fn is_user_allowed_to_upload_videos(_deps: &Deps, user_id: Uuid) -> bool {
    let allowed = true;
    tracing::Span::current().record("allowed", allowed);
    allowed
}
