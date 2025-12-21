use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ApiError {
    #[error("DB error {0}")]
    DataBaseError(#[from] DBError),
}

#[derive(Error, Debug)]
pub enum DBInfraError {
    #[error("Failed to init pg pool {0}")]
    FailedToInitPGPool(#[source] sqlx::Error),

    #[error("Failed to ping pg pool {0}")]
    FailedToPingPG(#[source] sqlx::Error),

    #[error("Failed to acquire pg pool {0}")]
    FailedToAcquirePG(#[source] sqlx::Error),
}

#[derive(Error, Debug)]
pub enum DBError {
    #[error("DB infra error {0}")]
    InfraError(#[from] DBInfraError),

    #[error("Failed to create user {0}")]
    FailedToCreateUser(#[source] sqlx::Error),

    #[error("Failed to get user {0}")]
    FailedToGetUser(#[source] sqlx::Error),

    #[error("Failed to update user {0}")]
    FailedToUpdateUser(#[source] sqlx::Error),

    #[error("Failed to delete user {0}")]
    FailedToDeleteUser(#[source] sqlx::Error),
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        match self {
            ApiError::DataBaseError(db_error) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("db error happened: {db_error}"),
            )
                .into_response(),
        }
    }
}
