use std::sync::Arc;

use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::post;
use axum::{middleware, Extension, Json, Router};
use repaint_server_usecase::model::admin::{AddOperatorRequest, RegisterRequest, SendEmailRequest};
use repaint_server_usecase::usecase::admin::AdminUsecase;

use crate::middleware::auth::auth;
use crate::routes::recover::Error;

pub mod event;

pub fn admin(usecase: impl AdminUsecase) -> Router {
    let usecase = Arc::new(usecase);

    Router::new()
        .route("/register", post(register))
        .route(
            "/send-email",
            post(send_email).route_layer(middleware::from_fn(auth)),
        )
        .route(
            "/add-operator",
            post(add_operator).route_layer(middleware::from_fn(auth)),
        )
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

async fn send_email<U: AdminUsecase>(
    State(usecase): State<&Arc<U>>,
    Extension(subject): Extension<String>,
    Json(req): Json<SendEmailRequest>,
) -> Result<impl IntoResponse, Error> {
    let usecase = Arc::clone(usecase);
    let _ = usecase.send_email(subject, req.event_id, req.email).await?;

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
