use axum::http::{Request, StatusCode};
use axum::middleware::Next;
use axum::response::Response;
use tracing::info;

pub async fn access_log<B>(req: Request<B>, next: Next<B>) -> Result<Response, StatusCode> {
    if req.uri() != "/metrics" {
        info!("{} {}", req.method(), req.uri());
    }
    Ok(next.run(req).await)
}
