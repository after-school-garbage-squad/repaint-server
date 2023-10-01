use axum::Router;
use tower_http::services::ServeFile;

pub fn license() -> Router {
    Router::new().nest_service("/", ServeFile::new("../../asset/license.html"))
}
