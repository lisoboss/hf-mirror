use anyhow::Error as anyhowError;
use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use git2::{Error as git2Error, ErrorCode as git2ErrorCode};
use std::io;

#[derive(Debug, PartialEq, Clone)]
struct HttpError(StatusCode, String);

impl From<&io::Error> for HttpError {
    fn from(error: &io::Error) -> Self {
        match error.kind() {
            io::ErrorKind::NotFound => Self(
                StatusCode::NOT_FOUND,
                format!("Resource not found: {}", error),
            ),
            _ => Self(
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("App io error: {}", error),
            ),
        }
    }
}

impl From<&git2Error> for HttpError {
    fn from(error: &git2Error) -> Self {
        match error.code() {
            git2ErrorCode::NotFound => Self(
                StatusCode::NOT_FOUND,
                format!("git repo not found: {}", error),
            ),
            _ => Self(
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("App git error: {}", error),
            ),
        }
    }
}

impl From<anyhowError> for HttpError {
    fn from(error: anyhowError) -> Self {
        if let Some(err) = error.downcast_ref::<io::Error>() {
            return err.into();
        }

        if let Some(err) = error.downcast_ref::<git2Error>() {
            return err.into();
        }

        Self(
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("App error: {}", error),
        )
    }
}

// Make our own error that wraps `anyhow::Error`.
#[derive(Debug)]
pub(crate) struct AppError(anyhowError);

// Tell axum how to convert `AppError` into a response.
impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let HttpError(code, err) = self.0.into();
        (code, err).into_response()
    }
}

// This enables using `?` on functions that return `Result<_, anyhow::Error>` to turn them into
// `Result<_, AppError>`. That way you don't need to do that manually.
impl<E> From<E> for AppError
where
    E: Into<anyhowError>,
{
    fn from(err: E) -> Self {
        Self(err.into())
    }
}
