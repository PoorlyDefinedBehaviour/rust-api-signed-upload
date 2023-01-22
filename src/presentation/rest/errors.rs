//! Contains functions to make it easier to deal with
//! errors that may be returned to the user.

use axum::response::IntoResponse;
use hyper::StatusCode;

#[tracing::instrument(name = "rest::errors::error_into_response", skip_all, fields(
    error = ?error
))]
pub fn error_into_response(error: anyhow::Error) -> axum::response::Response {
    // TODO: downcast anyhow::Error and decide which http status code to use
    // based on the real error.
    (StatusCode::INTERNAL_SERVER_ERROR, "Internal server error").into_response()
}
