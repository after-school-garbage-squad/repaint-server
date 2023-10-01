use axum::middleware;
use axum::routing::get;
use axum::Router;
use metrics_exporter_prometheus::{Matcher, PrometheusBuilder, PrometheusHandle};

use crate::middleware::access_log::access_log;
use crate::middleware::track_metrics::track_metrics;

pub fn metrics() -> Router {
    let recorder_handle = setup_metrics_recorder();

    Router::new()
        .route("/metrics", get(|| async move { recorder_handle.render() }))
        .layer(middleware::from_fn(track_metrics))
        .layer(middleware::from_fn(access_log))
}

fn setup_metrics_recorder() -> PrometheusHandle {
    const EXPONENTIAL_SECONDS: &[f64] = &[
        0.005, 0.01, 0.025, 0.05, 0.1, 0.25, 0.5, 1.0, 2.5, 5.0, 10.0,
    ];

    PrometheusBuilder::new()
        .set_buckets_for_metric(
            Matcher::Full("repaint-server_http_requests_duration_seconds".to_string()),
            EXPONENTIAL_SECONDS,
        )
        .expect("failed to set buckets for metric")
        .install_recorder()
        .expect("failed to install recorder")
}
