use std::backtrace::Backtrace;

use repaint_server_model::StaticError;

#[derive(Debug)]
pub enum Error {
    UnAuthorized,
    NotFound,
    BadRequest { message: String },
    Conflict,
    InternalServerError(anyhow::Error),
}

type Bt = Backtrace; // workaround to prevent thiserror to use unstable feature

#[derive(Debug, thiserror::Error)]
#[error("{error}\n{backtrace}")]
struct WithStackTrace<T> {
    error: T,
    backtrace: Bt,
}

impl<T> WithStackTrace<T> {
    #[track_caller]
    fn new(error: T) -> Self {
        Self {
            error,
            backtrace: Bt::force_capture(),
        }
    }
}

impl Error {
    #[track_caller]
    pub fn internal_server_error(e: impl StaticError) -> Self {
        Self::InternalServerError(WithStackTrace::new(e).into())
    }

    pub fn bad_request(message: impl Into<String>) -> Self {
        Self::BadRequest {
            message: message.into(),
        }
    }
}

impl<E: StaticError> From<E> for Error {
    #[track_caller]
    fn from(value: E) -> Self {
        Self::internal_server_error(value)
    }
}
