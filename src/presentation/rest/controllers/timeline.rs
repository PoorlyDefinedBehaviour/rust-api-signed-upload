use axum::extract::Query;
use axum::{Extension, Json};
use serde::Deserialize;
use std::sync::Arc;
use tracing::error;

use crate::domain::constants::TIMELINE_LIMIT;
use crate::domain::queries::timeline::get_timeline::Post;
use crate::domain::{contracts::deps::Deps, queries, value_objects::cursor::Cursor};
use crate::presentation::rest::errors::error_into_response;
use crate::presentation::rest::extensions::context::ExtractContext;

#[derive(Debug, Deserialize)]
pub struct GetTimelineQuery {
    cursor: i64,
}

#[tracing::instrument(name = "GET /v1/timeline", skip_all, fields(
    payload = ?payload,
    ctx = ?ctx
))]
pub async fn get_timeline(
    Query(payload): Query<GetTimelineQuery>,
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

            Err(error_into_response(error))
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::domain::constants::X_REQUEST_ID_HEADER_NAME;
    use crate::infra::factory;
    use crate::presentation::rest::{deps, router};
    use axum::{body::Body, http::Request};
    use rand::Rng;
    use tower::ServiceExt;

    #[tokio::test]
    #[ignore]
    async fn generate_post_test_data() -> Result<(), Box<dyn std::error::Error>> {
        dotenv::dotenv().ok();

        let mut executor = deps()?.db.write().await?;

        // TODO: this is sequential, fix it.
        for _ in 0..100 {
            let user_id = factory::user::create(&mut executor).await?;

            let num_posts = rand::thread_rng().gen_range(0..10);

            for _ in 0..num_posts {
                factory::post::create_for_user(user_id, &mut executor).await?;
            }
        }

        Ok(())
    }

    #[tokio::test]
    async fn can_get_timeline() -> Result<(), Box<dyn std::error::Error>> {
        dotenv::dotenv().ok();

        let mut executor = deps()?.db.write().await?;

        let user_id = factory::user::create(&mut executor).await?;

        factory::post::create_for_user(user_id, &mut executor).await?;

        // let posts = factory::post::create_many(1, &mut executor).await?;

        let app = router();

        let req = Request::builder()
            .method("GET")
            .uri("/v1/timeline?cursor=0")
            .header("Content-Type", "application/json")
            .header(X_REQUEST_ID_HEADER_NAME, 1)
            .body(Body::empty())?;

        let response = app.oneshot(req).await?;

        assert!(response.status().is_success());

        Ok(())
    }
}
