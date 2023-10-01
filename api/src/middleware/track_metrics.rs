use std::time::Instant;

use axum::http::Request;
use axum::middleware::Next;
use axum::response::IntoResponse;
use metrics::{histogram, increment_counter};

pub async fn track_metrics<B>(req: Request<B>, next: Next<B>) -> impl IntoResponse {
    let start = Instant::now();
    let path = req.uri().path().to_owned();
    let method = req.method().clone();
    let response = next.run(req).await;
    let latency = start.elapsed().as_secs_f64();
    let status = response.status().as_u16().to_string();
    let labels = [
        ("method", method.to_string()),
        ("path", path),
        ("status", status),
    ];
    increment_counter!("repaint-server_http_requests_total", &labels);
    histogram!(
        "repaint-server_http_requests_duration_seconds",
        latency,
        &labels
    );

    response
}
