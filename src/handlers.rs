use crate::app::AuthApp;
use crate::delivery_http::dto::{RegisterRequest, UpdateUserRequest};
use crate::errors::ApiError;
use axum::Json;
use axum::extract::{Path, State};
use axum::response::IntoResponse;
use std::sync::Arc;
use uuid::Uuid;

pub async fn create_user(
    State(app): State<Arc<AuthApp>>,
    payload: Json<RegisterRequest>,
) -> Result<impl IntoResponse, ApiError> {
    app.delivery.create_user(payload).await
}

pub async fn update_user(
    State(app): State<Arc<AuthApp>>,
    id: Path<Uuid>,
    payload: Json<UpdateUserRequest>,
) -> Result<impl IntoResponse, ApiError> {
    app.delivery.update_user(id, payload).await
}

pub async fn delete_user(
    State(app): State<Arc<AuthApp>>,
    payload: Path<Uuid>,
) -> Result<impl IntoResponse, ApiError> {
    app.delivery.delete_user(payload).await
}

pub async fn get_user(
    State(app): State<Arc<AuthApp>>,
    payload: Path<Uuid>,
) -> Result<impl IntoResponse, ApiError> {
    app.delivery.get_user(payload).await
}
