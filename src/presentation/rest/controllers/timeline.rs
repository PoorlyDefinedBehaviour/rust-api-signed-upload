use std::sync::Arc;
use axum::extract::Query;
use axum::{response::IntoResponse, Extension, Json};
use hyper::StatusCode;
use serde::{Deserialize};
use tracing::error;

use crate::domain::constants::TIMELINE_LIMIT;
use crate::domain::queries::timeline::get_timeline::Post;
use crate::presentation::rest::extensions::context::ExtractContext;
use crate::domain::{
    queries,
    contracts::deps::Deps, value_objects::cursor::Cursor
};

#[derive(Debug, Deserialize)]
pub struct GetTimelineViewModel {
    cursor: i64,
}

#[tracing::instrument(name = "controllers::get_timeline", skip_all, fields(
    payload = ?payload,
    ctx = ?ctx
))]
pub async fn get_timeline(
    Query(payload): Query<GetTimelineViewModel>,
    Extension(deps): Extension<Arc<Deps>>,
    ExtractContext(ctx): ExtractContext,
) -> Result<Json<Vec<Post>>, axum::response::Response> {
    let cursor = Cursor {
        offset: payload.cursor,
        limit: TIMELINE_LIMIT,
    };

    match queries::timeline::get_timeline::handle(&deps, &ctx, cursor).await {
        Ok(posts) => Ok(Json(posts)),
        Err(error) => {
            error!(?error, "unable to fetch timeline");
            Err((StatusCode::INTERNAL_SERVER_ERROR, "Internal server error").into_response())
        }
    }
}

#[cfg(test)]
mod tests {
    use axum::{http::Request, body::Body};
    use tower::ServiceExt;
    use crate::domain::queries::timeline::get_timeline::Post;
    use crate::presentation::rest::{router, deps};
    use crate::infra::factory;
    use crate::domain::constants::X_REQUEST_ID_HEADER_NAME;

    #[tokio::test]
    async fn can_get_timeline() -> Result<(), Box<dyn std::error::Error>> {
        dotenv::dotenv().ok();

        let mut executor = deps()?.db.write().await?;

        factory::timeline::refresh(&mut executor).await?;

        let posts = factory::timeline::create_many(1, &mut executor).await?;

        let app = router();

        let req = Request::builder()
            .method("GET")
            .uri("/v1/timeline?cursor=1")
            .header("Content-Type", "application/json")
            .header(X_REQUEST_ID_HEADER_NAME, 1)
            .body(Body::empty())?;

        let response = app
            .oneshot(req)
            .await?;

        let content: Vec<Post> = serde_json::from_slice(
            &hyper::body::to_bytes(response.into_body()).await?
        )?;

        assert_eq!(posts, content);

        Ok(())
    }
}
