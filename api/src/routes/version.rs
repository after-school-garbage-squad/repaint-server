use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::get;
use axum::Router;

use crate::routes::recover::Error;

#[cfg(debug_assertions)]
const VERSION: &str = "DEVELOPMENT BUILD";

#[cfg(not(debug_assertions))]
const VERSION: &str = env!("GIT_HASH");
#[cfg(not(debug_assertions))]
static_assertions::const_assert!(VERSION.len() == 7);

pub fn version() -> Router {
    Router::new().route("/", get(version_handler))
}

async fn version_handler() -> Result<impl IntoResponse, Error> {
    Ok((StatusCode::OK, VERSION))
}
