use crate::app::AuthApp;
use crate::delivery_http::dto::RegisterRequest;
use crate::errors::ApiError;
use axum::Json;
use axum::extract::State;
use axum::response::IntoResponse;
use std::sync::Arc;

pub async fn create_user(
    State(app): State<Arc<AuthApp>>,
    payload: Json<RegisterRequest>,
) -> Result<impl IntoResponse, ApiError> {
    app.delivery.create_user(payload).await
}
