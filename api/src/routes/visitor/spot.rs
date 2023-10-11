use std::sync::Arc;

use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::post;
use axum::{Json, Router};
use repaint_server_model::id::Id;
use repaint_server_model::visitor::Visitor;
use repaint_server_usecase::model::spot::ScannedRequest;
use repaint_server_usecase::model::visitor::VisitorIdentification;
use repaint_server_usecase::usecase::spot::SpotUsecase;

use crate::routes::recover::Error;

pub fn spot(usecase: impl SpotUsecase) -> Router {
    let usecase = Arc::new(usecase);

    Router::new()
        .route("/scanned", post(scanned))
        .with_state(usecase)
}

async fn scanned<U: SpotUsecase>(
    State(usecase): State<Arc<U>>,
    Path(visitor_id): Path<Id<Visitor>>,
    Json(req): Json<ScannedRequest>,
) -> Result<impl IntoResponse, Error> {
    let usecase = Arc::clone(&usecase);
    let res = usecase
        .scanned(
            VisitorIdentification {
                visitor_id,
                event_id: req.event_id,
            },
            req.hw_id,
        )
        .await?;

    Ok((StatusCode::OK, Json(res)))
}
