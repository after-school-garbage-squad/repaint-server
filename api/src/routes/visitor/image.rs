use std::sync::Arc;

use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::{get, post};
use axum::{Json, Router};
use repaint_server_model::id::Id;
use repaint_server_model::visitor::Visitor;
use repaint_server_usecase::model::image::{
    GetCurrentRequest, ListRequest, ProxyCurrentRequest, SetCurrentRequest,
};
use repaint_server_usecase::model::visitor::VisitorIdentification;
use repaint_server_usecase::usecase::image::ImageUsecase;

use crate::routes::recover::Error;

pub fn image(usecase: impl ImageUsecase) -> Router {
    let usecase = Arc::new(usecase);

    Router::new()
        .route("/get-current", get(get_current))
        .route("/list", get(list))
        .route("/proxy", get(proxy))
        .route("/set-current", post(set_current))
        .with_state(usecase)
}

async fn get_current<U: ImageUsecase>(
    State(usecase): State<Arc<U>>,
    Path(visitor_id): Path<Id<Visitor>>,
    Json(req): Json<GetCurrentRequest>,
) -> Result<impl IntoResponse, Error> {
    let usecase = Arc::clone(&usecase);
    let res = usecase
        .get_current_image(VisitorIdentification {
            visitor_id,
            event_id: req.event_id,
        })
        .await?;

    Ok((StatusCode::OK, Json(res)))
}

async fn list<U: ImageUsecase>(
    State(usecase): State<Arc<U>>,
    Path(visitor_id): Path<Id<Visitor>>,
    Json(req): Json<ListRequest>,
) -> Result<impl IntoResponse, Error> {
    let usecase = Arc::clone(&usecase);
    let res = usecase
        .list_image(VisitorIdentification {
            visitor_id,
            event_id: req.event_id,
        })
        .await?;

    Ok((StatusCode::OK, Json(res)))
}

async fn proxy<U: ImageUsecase>(
    State(usecase): State<Arc<U>>,
    Path(visitor_id): Path<Id<Visitor>>,
    Json(req): Json<ProxyCurrentRequest>,
) -> Result<impl IntoResponse, Error> {
    let usecase = Arc::clone(&usecase);
    let res = usecase
        .proxy_current_image(req.event_id, req.image_id, visitor_id)
        .await?;

    Ok((StatusCode::OK, Json(res)))
}

async fn set_current<U: ImageUsecase>(
    State(usecase): State<Arc<U>>,
    Path(visitor_id): Path<Id<Visitor>>,
    Json(req): Json<SetCurrentRequest>,
) -> Result<impl IntoResponse, Error> {
    let usecase = Arc::clone(&usecase);
    let _ = usecase
        .set_current_image(
            VisitorIdentification {
                visitor_id,
                event_id: req.event_id,
            },
            req.image_id,
        )
        .await;

    Ok(StatusCode::NO_CONTENT)
}
