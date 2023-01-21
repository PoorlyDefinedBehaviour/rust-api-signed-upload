#[tracing::instrument(name = "GET /v1/health-check", skip_all)]
pub async fn handle() -> &'static str {
  "OK"
}

#[cfg(test)]
mod tests {
  use axum::{
    body::Body,
    http::{Request, StatusCode},
  };
  use tower::ServiceExt;

  use crate::presentation::rest::router;

  #[tokio::test]
  async fn health_check() -> Result<(), Box<dyn std::error::Error>> {
    let app = router();

    let response = app
      .oneshot(Request::builder().uri("/v1/health-check").body(Body::empty())?)
      .await?;

    assert_eq!(response.status(), StatusCode::OK);

    Ok(())
  }
}
