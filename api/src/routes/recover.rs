use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;
use repaint_server_usecase::usecase::error::Error as UsecaseError;
use serde_json::json;

#[derive(Debug)]
pub struct Error(UsecaseError);

impl From<UsecaseError> for Error {
    fn from(e: UsecaseError) -> Self {
        Self(e)
    }
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            Error(UsecaseError::BadRequest { message }) => {
                let msg: &str = &message;

                (StatusCode::BAD_REQUEST, msg)
            }
            Error(UsecaseError::InternalServerError(e)) => {
                tracing::error!("internal server error: {:?}", e);

                (StatusCode::INTERNAL_SERVER_ERROR, "Internal Server Error")
            }
            Error(UsecaseError::NotFound) => (StatusCode::NOT_FOUND, "Not Found"),
            Error(UsecaseError::UnAuthorized) => (StatusCode::UNAUTHORIZED, "UnAuthorized"),
        };
        let body = Json(json!({
            "error": error_message,
        }));

        (status, body).into_response()
    }
}
