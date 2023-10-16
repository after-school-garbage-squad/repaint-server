use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;
use repaint_server_usecase::usecase::error::Error as UsecaseError;
use serde_json::json;

#[derive(Debug)]
pub struct Error(pub UsecaseError);

impl From<UsecaseError> for Error {
    fn from(e: UsecaseError) -> Self {
        Self(e)
    }
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            Error(UsecaseError::BadRequest { message }) => (StatusCode::BAD_REQUEST, message),
            Error(UsecaseError::Conflict) => (StatusCode::CONFLICT, "Conflict".to_string()),
            Error(UsecaseError::InternalServerError(e)) => {
                tracing::error!("internal server error: {:?}", e);

                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Internal Server Error".to_string(),
                )
            }
            Error(UsecaseError::NotFound) => (StatusCode::NOT_FOUND, "Not Found".to_string()),
            Error(UsecaseError::RangeNotSatisfiable) => (
                StatusCode::RANGE_NOT_SATISFIABLE,
                "Range Not Satisfiable".to_string(),
            ),
            Error(UsecaseError::UnAuthorized) => {
                (StatusCode::UNAUTHORIZED, "UnAuthorized".to_string())
            }
        };
        let body = Json(json!({
            "error": error_message,
        }));

        (status, body).into_response()
    }
}
