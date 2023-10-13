use std::sync::Arc;

use axum::extract::{Path, Query, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::{get, post};
use axum::{Json, Router};
use repaint_server_model::id::Id;
use repaint_server_model::visitor::Visitor;
use repaint_server_usecase::model::palette::{CheckPalettesCompletedQuery, PickRequest};
use repaint_server_usecase::model::visitor::VisitorIdentification;
use repaint_server_usecase::usecase::palette::PaletteUsecase;

use crate::routes::recover::Error;

pub fn palette(usecase: impl PaletteUsecase) -> Router {
    let usecase = Arc::new(usecase);

    Router::new()
        .route("/pick", post(pick))
        .route("/complete", get(complete))
        .with_state(usecase)
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

async fn complete<U: PaletteUsecase>(
    State(usecase): State<Arc<U>>,
    Path(visitor_id): Path<Id<Visitor>>,
    Query(q): Query<CheckPalettesCompletedQuery>,
) -> Result<impl IntoResponse, Error> {
    let usecase = Arc::clone(&usecase);
    let res = usecase
        .check_palettes_completed(VisitorIdentification {
            visitor_id,
            event_id: q.event_id,
        })
        .await?;

    Ok((StatusCode::OK, Json(res)))
}
