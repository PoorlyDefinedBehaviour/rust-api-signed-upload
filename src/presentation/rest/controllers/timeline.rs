use std::sync::Arc;
use axum::{response::IntoResponse, Extension, Json};
use hyper::StatusCode;
use tracing::error;

use crate::domain::constants::TIMELINE_LIMIT;
use crate::domain::queries::timeline::get_timeline::Post;
use crate::presentation::rest::extensions::context::ExtractContext;
use crate::domain::{
    queries,
    contracts::deps::Deps, value_objects::cursor::Cursor
};

struct GetTimelineViewModel {
    cursor: i128,
}

pub async fn get_timeline(
    Json(payload): Json<GetTimelineViewModel>,
    Extension(deps): Extension<Arc<Deps>>,
    ExtractContext(ctx): ExtractContext,
) -> Result<(StatusCode, Vec<Post>), axum::response::Response> {
    let cursor = Cursor {
        offset: payload.cursor,
        limit: TIMELINE_LIMIT,
    };

    match queries::timeline::get_timeline::handle(&deps, &ctx, cursor).await {
        Ok(posts) => Ok((StatusCode::OK, posts)),
        Err(error) => {
            error!(?error, "unable to fetch timeline");
            Err((StatusCode::INTERNAL_SERVER_ERROR, "Internal server error").into_response())
        }
    }
}
