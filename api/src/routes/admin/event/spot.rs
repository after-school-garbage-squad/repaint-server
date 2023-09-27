use std::sync::Arc;

use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::{delete as delete_handler, get, patch, post};
use axum::{middleware, Extension, Json, Router};
use repaint_server_model::event::Event;
use repaint_server_model::id::Id;
use repaint_server_usecase::model::spot::{
    Beacon, CheckByBeaconRequest, CheckByQrRequest, DeleteRequest, RegisterRequest, UpdateRequest,
};
use repaint_server_usecase::usecase::spot::SpotUsecase;

use crate::middleware::auth::auth;
use crate::routes::recover::Error;

pub fn spot(usecase: impl SpotUsecase) -> Router {
    let usecase = Arc::new(usecase);

    Router::new()
        .route("/check-by-beacon", get(check_by_beacon))
        .route("/check-by-qr", get(check_by_qr))
        .route("/delete", delete_handler(delete))
        .route("/list", get(list))
        .route("/register", post(register))
        .route("/update", patch(update))
        .layer(middleware::from_fn(auth))
        .with_state(usecase)
}

async fn check_by_beacon<U: SpotUsecase>(
    State(usecase): State<Arc<U>>,
    Extension(subject): Extension<String>,
    Path(event_id): Path<Id<Event>>,
    Json(req): Json<CheckByBeaconRequest>,
) -> Result<impl IntoResponse, Error> {
    let usecase = Arc::clone(&usecase);
    let res = usecase
        .check_status_by_beacon(subject, event_id, req.hw_id)
        .await?;

    Ok((StatusCode::OK, Json(res)))
}

async fn check_by_qr<U: SpotUsecase>(
    State(usecase): State<Arc<U>>,
    Extension(subject): Extension<String>,
    Path(event_id): Path<Id<Event>>,
    Json(req): Json<CheckByQrRequest>,
) -> Result<impl IntoResponse, Error> {
    let usecase = Arc::clone(&usecase);
    let res = usecase
        .check_status_by_qr(subject, event_id, req.spot_id)
        .await?;

    Ok((StatusCode::OK, Json(res)))
}

async fn delete<U: SpotUsecase>(
    State(usecase): State<Arc<U>>,
    Extension(subject): Extension<String>,
    Path(event_id): Path<Id<Event>>,
    Json(req): Json<DeleteRequest>,
) -> Result<impl IntoResponse, Error> {
    let usecase = Arc::clone(&usecase);
    let _ = usecase.delete_spot(subject, event_id, req.spot_id).await?;

    Ok(StatusCode::NO_CONTENT)
}

async fn list<U: SpotUsecase>(
    State(usecase): State<Arc<U>>,
    Extension(subject): Extension<String>,
    Path(event_id): Path<Id<Event>>,
) -> Result<impl IntoResponse, Error> {
    let usecase = Arc::clone(&usecase);
    let res = usecase.list_spot(subject, event_id).await?;

    Ok((StatusCode::OK, Json(res)))
}

async fn register<U: SpotUsecase>(
    State(usecase): State<Arc<U>>,
    Extension(subject): Extension<String>,
    Path(event_id): Path<Id<Event>>,
    Json(req): Json<RegisterRequest>,
) -> Result<impl IntoResponse, Error> {
    let usecase = Arc::clone(&usecase);
    let res = usecase
        .register_spot(
            subject,
            event_id,
            req.name,
            Beacon {
                hw_id: req.hw_id,
                service_uuid: req.service_uuid,
            },
        )
        .await?;

    Ok((StatusCode::CREATED, Json(res)))
}

async fn update<U: SpotUsecase>(
    State(usecase): State<Arc<U>>,
    Extension(subject): Extension<String>,
    Path(event_id): Path<Id<Event>>,
    Json(req): Json<UpdateRequest>,
) -> Result<impl IntoResponse, Error> {
    let usecase = Arc::clone(&usecase);
    let res = usecase
        .update_spot(subject, event_id, req.spot_id, req.name, req.is_pick)
        .await?;

    Ok((StatusCode::OK, Json(res)))
}
