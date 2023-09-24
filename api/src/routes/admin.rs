use std::sync::Arc;

use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::post;
use axum::{Extension, Json, Router};
use repaint_server_usecase::model::admin::{AddOperatorRequest, RegisterRequest};
use repaint_server_usecase::usecase::admin::AdminUsecase;

use crate::routes::recover::Error;

pub fn admin(usecase: impl AdminUsecase) -> Router {
    let usecase = Arc::new(usecase);

    Router::new()
        .route("/register", post(register))
        .route("/add-operator", post(add_operator))
        .with_state(&usecase)
}

async fn register<U: AdminUsecase>(
    State(usecase): State<&Arc<U>>,
    Json(req): Json<RegisterRequest>,
) -> Result<impl IntoResponse, Error> {
    let usecase = Arc::clone(usecase);
    let _ = usecase.add_admin(req.subject).await?;

    Ok(StatusCode::NO_CONTENT)
}

async fn add_operator<U: AdminUsecase>(
    State(usecase): State<&Arc<U>>,
    Extension(subject): Extension<String>,
    Json(req): Json<AddOperatorRequest>,
) -> Result<impl IntoResponse, Error> {
    let usecase = Arc::clone(usecase);
    let _ = usecase.add_operator(subject, req.token).await?;

    Ok(StatusCode::NO_CONTENT)
}
