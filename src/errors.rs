use axum::Json;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use serde_json::json;
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
    DBDerivedError(#[from] DBError),
    #[error("Failed to hash password {0}")]
    HashPasswordError(#[from] argon2::password_hash::Error),
    #[error("User not found")]
    UserNotFoundError,
    #[error("Invalid credentials")]
    InvalidCreds,
    #[error("Session already exists")]
    SessionAlreadyExists,
}

impl UsecaseError {
    fn status_code(&self) -> StatusCode {
        match self {
            UsecaseError::DBDerivedError(err) => err.status_code(),
            UsecaseError::HashPasswordError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            UsecaseError::UserNotFoundError => StatusCode::NOT_FOUND,
            UsecaseError::InvalidCreds | UsecaseError::SessionAlreadyExists => StatusCode::CONFLICT,
        }
    }
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

    #[error("Failed to create session {0}")]
    FailedToCreateSession(#[source] deadpool_redis::redis::RedisError),

    #[error("Failed to get user from session {0}")]
    FailedToGetUserFromSession(#[source] deadpool_redis::redis::RedisError),

    #[error("Failed to delete session {0}")]
    FailedToDeleteSession(#[source] deadpool_redis::redis::RedisError),

    #[error("Session not found")]
    SessionNotFound,

    #[error("Session user not found")]
    SessionUserNotFound,

    #[error("Failed to parse UUID {0}")]
    FailedToParseUUID(#[from] uuid::Error),
}

impl DBError {
    fn status_code(&self) -> StatusCode {
        match self {
            DBError::SessionNotFound => StatusCode::NOT_FOUND,
            DBError::SessionUserNotFound => StatusCode::NOT_FOUND,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let (code, err_body) = match self {
            ApiError::DataBaseError(err) => (err.status_code(), err.to_string()),

            ApiError::UseCaseError(err) => (err.status_code(), err.to_string()),
        };

        let json_body = Json(json!({
            "error": err_body
        }));

        (code, json_body).into_response()
    }
}
