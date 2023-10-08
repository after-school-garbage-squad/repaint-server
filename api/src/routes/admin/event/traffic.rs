use std::sync::Arc;

use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::{get, post};
use axum::{middleware, Extension, Json, Router};
use repaint_server_model::event::Event;
use repaint_server_model::id::Id;
use repaint_server_usecase::model::traffic::{DisableBonusRequest, EnableBonusRequest};
use repaint_server_usecase::usecase::traffic::TrafficUsecase;

use crate::middleware::auth::auth;
use crate::routes::recover::Error;

pub fn traffic(usecase: impl TrafficUsecase) -> Router {
    let usecase = Arc::new(usecase);

    Router::new()
        .route("/enable-bonus", post(enable_bonus))
        .route("/disable-bonus", post(disable_bonus))
        .route("/get-status", get(get_status))
        .layer(middleware::from_fn(auth))
        .with_state(usecase)
}

async fn enable_bonus<U: TrafficUsecase>(
    State(usecase): State<Arc<U>>,
    Extension(subject): Extension<String>,
    Path(event_id): Path<Id<Event>>,
    Json(req): Json<EnableBonusRequest>,
) -> Result<impl IntoResponse, Error> {
    let usecase = Arc::clone(&usecase);
    let _ = usecase
        .enable_bonus(subject, event_id, req.from, req.to)
        .await?;

    Ok(StatusCode::NO_CONTENT)
}

async fn disable_bonus<U: TrafficUsecase>(
    State(usecase): State<Arc<U>>,
    Extension(subject): Extension<String>,
    Path(event_id): Path<Id<Event>>,
    Json(req): Json<DisableBonusRequest>,
) -> Result<impl IntoResponse, Error> {
    let usecase = Arc::clone(&usecase);
    let _ = usecase
        .disable_bonus(subject, event_id, req.spot_id)
        .await?;

    Ok(StatusCode::NO_CONTENT)
}

async fn get_status<U: TrafficUsecase>(
    State(usecase): State<Arc<U>>,
    Extension(subject): Extension<String>,
    Path(event_id): Path<Id<Event>>,
) -> Result<impl IntoResponse, Error> {
    let usecase = Arc::clone(&usecase);
    let res = usecase.get_traffic_status(subject, event_id).await?;

    Ok((StatusCode::OK, Json(res)))
}
