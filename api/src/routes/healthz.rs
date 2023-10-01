use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::get;
use axum::Router;

use crate::routes::recover::Error;

pub fn healthz() -> Router {
    Router::new().route("/healthz", get(hc))
}

async fn hc() -> Result<impl IntoResponse, Error> {
    tracing::debug!("Access health check endpoint.");

    Ok(StatusCode::NO_CONTENT)
}
