use std::sync::Arc;

use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::{delete as delete_handler, get, patch, post};
use axum::{middleware, Extension, Json, Router};
use repaint_server_model::event::Event;
use repaint_server_model::id::Id;
use repaint_server_usecase::model::event::{CreateEventRequest, UpdateEventRequest};
use repaint_server_usecase::usecase::event::EventUsecase;

use crate::middleware::auth::auth;
use crate::routes::recover::Error;

pub fn event(usecase: impl EventUsecase) -> Router {
    let usecase = Arc::new(usecase);

    Router::new()
        .route("/create", post(create))
        .route("/:event_id/delete", delete_handler(delete))
        .route("/list", get(list))
        .route("/:event_id/update", patch(update))
        .layer(middleware::from_fn(auth))
        .with_state(&usecase)
}

async fn create<U: EventUsecase>(
    State(usecase): State<&Arc<U>>,
    Extension(subject): Extension<String>,
    Json(req): Json<CreateEventRequest>,
) -> Result<impl IntoResponse, Error> {
    let usecase = Arc::clone(usecase);
    let res = usecase
        .create_event(subject, req.name, req.hp_url, req.contact)
        .await?;

    Ok((StatusCode::CREATED, Json(res)))
}

async fn delete<U: EventUsecase>(
    State(usecase): State<&Arc<U>>,
    Extension(subject): Extension<String>,
    Path(event_id): Path<Id<Event>>,
) -> Result<impl IntoResponse, Error> {
    let usecase = Arc::clone(usecase);
    let _ = usecase.delete_event(subject, event_id).await?;

    Ok(StatusCode::NO_CONTENT)
}

async fn list<U: EventUsecase>(
    State(usecase): State<&Arc<U>>,
    Extension(subject): Extension<String>,
) -> Result<impl IntoResponse, Error> {
    let usecase = Arc::clone(usecase);
    let res = usecase.list_event(subject).await?;

    Ok((StatusCode::OK, Json(res)))
}

async fn update<U: EventUsecase>(
    State(usecase): State<&Arc<U>>,
    Extension(subject): Extension<String>,
    Path(event_id): Path<Id<Event>>,
    Json(req): Json<UpdateEventRequest>,
) -> Result<impl IntoResponse, Error> {
    let usecase = Arc::clone(usecase);
    let res = usecase
        .update_event(subject, event_id, req.name, req.hp_url, req.contact)
        .await?;

    Ok((StatusCode::OK, Json(res)))
}
