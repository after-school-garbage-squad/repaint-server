use std::sync::Arc;

use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::post;
use axum::{Json, Router};
use repaint_server_model::id::Id;
use repaint_server_model::visitor::Visitor;
use repaint_server_usecase::model::palette::{DropRequest, PickRequest};
use repaint_server_usecase::model::visitor::VisitorIdentification;
use repaint_server_usecase::usecase::palette::PaletteUsecase;

use crate::routes::recover::Error;

pub fn palette(usecase: impl PaletteUsecase) -> Router {
    let usecase = Arc::new(usecase);

    Router::new()
        .route("/drop", post(drop))
        .route("/pick", post(pick))
        .with_state(usecase)
}

async fn drop<U: PaletteUsecase>(
    State(usecase): State<Arc<U>>,
    Path(visitor_id): Path<Id<Visitor>>,
    Json(req): Json<DropRequest>,
) -> Result<impl IntoResponse, Error> {
    let usecase = Arc::clone(&usecase);
    let _ = usecase
        .drop_palette(
            VisitorIdentification {
                visitor_id,
                event_id: req.event_id,
            },
            req.hw_id,
        )
        .await?;

    Ok(StatusCode::NO_CONTENT)
}

async fn pick<U: PaletteUsecase>(
    State(usecase): State<Arc<U>>,
    Path(visitor_id): Path<Id<Visitor>>,
    Json(req): Json<PickRequest>,
) -> Result<impl IntoResponse, Error> {
    let usecase = Arc::clone(&usecase);
    let _ = usecase
        .pick_palette(
            VisitorIdentification {
                visitor_id,
                event_id: req.event_id,
            },
            req.spot_id,
        )
        .await?;

    Ok(StatusCode::NO_CONTENT)
}
