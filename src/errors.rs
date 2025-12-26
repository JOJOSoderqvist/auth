use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ApiError {
    #[error("DB error {0}")]
    DataBaseError(#[from] DBError),

    #[error("Usecase error {0}")]
    UseCaseError(#[from] UsecaseError),
}

#[derive(Error, Debug)]
pub enum DBInfraError {
    #[error("Failed to init pg pool {0}")]
    FailedToInitPGPool(#[source] sqlx::Error),

    #[error("Failed to ping pg pool {0}")]
    FailedToPingPG(#[source] sqlx::Error),

    #[error("Failed to acquire pg pool {0}")]
    FailedToAcquirePG(#[source] sqlx::Error),

    #[error("Failed to init redis pool {0}")]
    FailedToInitRedisPool(#[from] deadpool_redis::CreatePoolError),
}

#[derive(Error, Debug)]
pub enum UsecaseError {
    // TODO: mb cringe
    #[error("db error {0}")]
    DBDerivedError(#[source] DBError),
    #[error("Failed to hash password {0}")]
    HashPasswordError(#[from] argon2::password_hash::Error),
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

    #[error("Failed to get redis pool connection {0}")]
    FailedToGetRedisPoolConn(#[from] deadpool_redis::PoolError),
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        match self {
            ApiError::DataBaseError(db_error) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("db error happened: {db_error}"),
            )
                .into_response(),

            ApiError::UseCaseError(usecase_error) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("usecase error happened: {usecase_error}"),
            )
                .into_response(),
        }
    }
}
