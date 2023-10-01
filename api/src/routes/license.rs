use axum::http::StatusCode;
use axum::response::{Html, IntoResponse};
use axum::routing::get;
use axum::Router;

use crate::routes::recover::Error;

pub fn license() -> Router {
    Router::new().route("/", get(license_handler))
}

async fn license_handler() -> Result<impl IntoResponse, Error> {
    let html = include_str!("./licenses.txt");

    Ok((StatusCode::OK, Html(html)))
}
