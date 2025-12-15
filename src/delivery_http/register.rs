use axum::http::StatusCode;
use crate::delivery_http::dto::{RegisterRequest};
use axum::response::IntoResponse;
use axum::Json;
use crate::errors::ApiError;

pub async fn register(Json(payload): Json<RegisterRequest>) -> Result<impl IntoResponse, ApiError> {
    Ok(())
}