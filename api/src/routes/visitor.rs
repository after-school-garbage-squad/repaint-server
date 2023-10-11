use std::sync::Arc;

use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::{delete as delete_handler, patch, post};
use axum::{Json, Router};
use repaint_server_model::id::Id;
use repaint_server_model::visitor::Visitor;
use repaint_server_usecase::model::visitor::{
    DeleteRequest, InitializeRequest, RegisterRequest, VisitorIdentification,
};
use repaint_server_usecase::usecase::image::ImageUsecase;
use repaint_server_usecase::usecase::palette::PaletteUsecase;
use repaint_server_usecase::usecase::spot::SpotUsecase;
use repaint_server_usecase::usecase::visitor::VisitorUsecase;

use crate::routes::recover::Error;

use self::image::image;
use self::palette::palette;
use self::spot::spot;

mod image;
mod palette;
mod spot;

pub fn visitor(
    visitor_usecase: impl VisitorUsecase,
    palette_usecase: impl PaletteUsecase,
    image_usecase: impl ImageUsecase,
    spot_usecase: impl SpotUsecase,
) -> Router {
    let palette = palette(palette_usecase);
    let image = image(image_usecase);
    let spot = spot(spot_usecase);
    let usecase = Arc::new(visitor_usecase);

    Router::new()
        .route("/:visotor_id/delete", delete_handler(delete))
        .route("/:visitor_id/initialize", patch(initialize))
        .route("/join", post(join))
        .with_state(usecase)
        .nest("/:visitor_id/palette", palette)
        .nest("/:visitor_id/image", image)
        .nest("/:visitor_id/spot", spot)
}

async fn delete<U: VisitorUsecase>(
    State(usecase): State<Arc<U>>,
    Path(visitor_id): Path<Id<Visitor>>,
    Json(req): Json<DeleteRequest>,
) -> Result<impl IntoResponse, Error> {
    let usecase = Arc::clone(&usecase);
    let _ = usecase
        .delete_visitor(VisitorIdentification {
            visitor_id,
            event_id: req.event_id,
        })
        .await?;

    Ok(StatusCode::NO_CONTENT)
}

async fn initialize<U: VisitorUsecase>(
    State(usecase): State<Arc<U>>,
    Path(visitor_id): Path<Id<Visitor>>,
    Json(req): Json<InitializeRequest>,
) -> Result<impl IntoResponse, Error> {
    let usecase = Arc::clone(&usecase);
    let res = usecase
        .initialize_visitor(
            VisitorIdentification {
                visitor_id,
                event_id: req.event_id,
            },
            req.registration_id,
        )
        .await?;

    Ok((
        StatusCode::OK,
        Json(serde_json::json!({
            "event": res.0,
            "visitor": res.1,
        })),
    ))
}

async fn join<U: VisitorUsecase>(
    State(usecase): State<Arc<U>>,
    Json(req): Json<RegisterRequest>,
) -> Result<impl IntoResponse, Error> {
    let usecase = Arc::clone(&usecase);
    let res = usecase
        .join_event(req.event_id, req.registration_id)
        .await?;

    Ok((
        StatusCode::OK,
        Json(serde_json::json!({
            "event": res.0,
            "visitor": res.1,
        })),
    ))
}
