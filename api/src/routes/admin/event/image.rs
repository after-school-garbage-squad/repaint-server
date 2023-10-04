use std::str::FromStr;
use std::sync::Arc;

use axum::extract::{Multipart, Path, Query, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::{delete, get, post};
use axum::{middleware, Extension, Json, Router};
use repaint_server_model::event::Event;
use repaint_server_model::event_image::Image as EventImage;
use repaint_server_model::id::Id;
use repaint_server_model::visitor::Visitor;
use repaint_server_usecase::model::image::DeleteDefaultRequest;
use repaint_server_usecase::usecase::error::Error as UsecaseError;
use repaint_server_usecase::usecase::image::ImageUsecase;

use crate::middleware::auth::auth;
use crate::routes::recover::Error;

pub fn image(usecase: impl ImageUsecase) -> Router {
    let usecase = Arc::new(usecase);

    Router::new()
        .route("/check_visitor", get(check_visitor))
        .route("/delete-default", delete(delete_default))
        .route("/register-default", post(register_default))
        .route("/upload-visitor", post(upload_visitor))
        .route("/proxy", get(proxy))
        .layer(middleware::from_fn(auth))
        .with_state(usecase)
}

async fn check_visitor<U: ImageUsecase>(
    State(usecase): State<Arc<U>>,
    Extension(subject): Extension<String>,
    Path(event_id): Path<Id<Event>>,
    Query(visitor_id): Query<Id<Visitor>>,
) -> Result<impl IntoResponse, Error> {
    let usecase = Arc::clone(&usecase);
    let res = usecase
        .check_visitor_image_exist(subject, event_id, visitor_id)
        .await?;

    Ok((StatusCode::OK, Json(res)))
}

async fn delete_default<U: ImageUsecase>(
    State(usecase): State<Arc<U>>,
    Extension(subject): Extension<String>,
    Path(event_id): Path<Id<Event>>,
    Json(req): Json<DeleteDefaultRequest>,
) -> Result<impl IntoResponse, Error> {
    let usecase = Arc::clone(&usecase);
    let _ = usecase
        .delete_default_image(subject, event_id, req.image_id)
        .await?;

    Ok(StatusCode::NO_CONTENT)
}

async fn register_default<U: ImageUsecase>(
    State(usecase): State<Arc<U>>,
    Extension(subject): Extension<String>,
    Path(event_id): Path<Id<Event>>,
    mut multipart: Multipart,
) -> Result<impl IntoResponse, Error> {
    let usecase = Arc::clone(&usecase);
    while let Some(field) = multipart
        .next_field()
        .await
        .map_err(|e| UsecaseError::InternalServerError(e.into()))?
    {
        let data = field
            .bytes()
            .await
            .map_err(|e| UsecaseError::InternalServerError(e.into()))?;
        let _ = usecase
            .add_default_image(subject.clone(), event_id, data.into())
            .await?;
    }

    Ok(StatusCode::NO_CONTENT)
}

async fn upload_visitor<U: ImageUsecase>(
    State(usecase): State<Arc<U>>,
    Extension(subject): Extension<String>,
    Path(event_id): Path<Id<Event>>,
    mut multipart: Multipart,
) -> Result<impl IntoResponse, Error> {
    let usecase = Arc::clone(&usecase);
    while let Some(field) = multipart
        .next_field()
        .await
        .map_err(|e| UsecaseError::InternalServerError(e.into()))?
    {
        let file_name = field.file_name().unwrap();
        let file_name = match file_name.ends_with(".png") {
            true => file_name.trim_end_matches(".png"),
            false => {
                return Err(Error(UsecaseError::BadRequest {
                    message: "Invalid file ext".to_string(),
                }))
            }
        };
        let visitor_id = Id::<Visitor>::from_str(file_name)
            .map_err(|e| UsecaseError::InternalServerError(e.into()))?;
        let data = field
            .bytes()
            .await
            .map_err(|e| UsecaseError::InternalServerError(e.into()))?;
        let _ = usecase
            .upload_visitor_image(subject.clone(), event_id, visitor_id, data.into())
            .await?;
    }

    Ok(StatusCode::NO_CONTENT)
}

async fn proxy<U: ImageUsecase>(
    State(usecase): State<Arc<U>>,
    Path(event_id): Path<Id<Event>>,
    Query(image_id): Query<Id<EventImage>>,
) -> Result<impl IntoResponse, Error> {
    let usecase = Arc::clone(&usecase);
    let res = usecase.proxy_event_image(event_id, image_id).await?;

    Ok((StatusCode::OK, Json(res)))
}
