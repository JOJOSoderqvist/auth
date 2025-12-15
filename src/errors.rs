use axum::response::{IntoResponse, Response};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ApiError {

}

#[derive(Error, Debug)]
pub enum DBError {
    #[error("Failed to init pg pool {0}")]
    FailedToInitPGPool(#[source] sqlx::Error)
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        todo!()
    }
}