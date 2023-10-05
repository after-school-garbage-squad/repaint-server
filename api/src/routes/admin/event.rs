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
use repaint_server_usecase::usecase::image::ImageUsecase;
use repaint_server_usecase::usecase::spot::SpotUsecase;
use repaint_server_usecase::usecase::traffic::TrafficUsecase;

use crate::middleware::auth::auth;
use crate::routes::recover::Error;

use self::image::image;
use self::spot::spot;
use self::traffic::traffic;

mod image;
mod spot;
mod traffic;

pub fn event(
    event_usecase: impl EventUsecase,
    traffic_usecase: impl TrafficUsecase,
    spot_usecase: impl SpotUsecase,
    image_usecase: impl ImageUsecase,
) -> Router {
    let traffic = traffic(traffic_usecase);
    let spot = spot(spot_usecase);
    let image = image(image_usecase);
    let usecase = Arc::new(event_usecase);

    Router::new()
        .route("/create", post(create))
        .route("/:event_id/delete", delete_handler(delete))
        .route("/list", get(list))
        .route("/:event_id/update", patch(update))
        .route("/:event_id/finish", post(finish))
        .layer(middleware::from_fn(auth))
        .with_state(usecase)
        .nest("/:event_id/traffic", traffic)
        .nest("/:event_id/spot", spot)
        .nest("/:event_id/image", image)
}

async fn create<U: EventUsecase>(
    State(usecase): State<Arc<U>>,
    Extension(subject): Extension<String>,
    Json(req): Json<CreateEventRequest>,
) -> Result<impl IntoResponse, Error> {
    let usecase = Arc::clone(&usecase);
    let res = usecase
        .create_event(subject, req.name, req.hp_url, req.contact)
        .await?;

    Ok((StatusCode::CREATED, Json(res)))
}

async fn delete<U: EventUsecase>(
    State(usecase): State<Arc<U>>,
    Extension(subject): Extension<String>,
    Path(event_id): Path<Id<Event>>,
) -> Result<impl IntoResponse, Error> {
    let usecase = Arc::clone(&usecase);
    let _ = usecase.delete_event(subject, event_id).await?;

    Ok(StatusCode::NO_CONTENT)
}

async fn list<U: EventUsecase>(
    State(usecase): State<Arc<U>>,
    Extension(subject): Extension<String>,
) -> Result<impl IntoResponse, Error> {
    let usecase = Arc::clone(&usecase);
    let res = usecase.list_event(subject).await?;

    Ok((StatusCode::OK, Json(res)))
}

async fn update<U: EventUsecase>(
    State(usecase): State<Arc<U>>,
    Extension(subject): Extension<String>,
    Path(event_id): Path<Id<Event>>,
    Json(req): Json<UpdateEventRequest>,
) -> Result<impl IntoResponse, Error> {
    let usecase = Arc::clone(&usecase);
    let res = usecase
        .update_event(subject, event_id, req.name, req.hp_url, req.contact)
        .await?;

    Ok((StatusCode::OK, Json(res)))
}

async fn finish<U: EventUsecase>(
    State(usecase): State<Arc<U>>,
    Extension(subject): Extension<String>,
    Path(event_id): Path<Id<Event>>,
) -> Result<impl IntoResponse, Error> {
    let usecase = Arc::clone(&usecase);
    let _ = usecase.finish_event(subject, event_id).await?;

    Ok(StatusCode::NO_CONTENT)
}
