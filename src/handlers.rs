use crate::app::AuthApp;
use crate::delivery_http::dto::{LoginRequest, RegisterRequest, UpdateUserRequest};
use crate::errors::ApiError;
use axum::Json;
use axum::extract::{Path, State};
use axum::response::IntoResponse;
use axum_extra::extract::CookieJar;
use std::sync::Arc;
use uuid::Uuid;

pub async fn create_user(
    State(app): State<Arc<AuthApp>>,
    jar: CookieJar,
    payload: Json<RegisterRequest>,
) -> Result<impl IntoResponse, ApiError> {
    app.http_delivery.create_user(jar, payload).await
}

pub async fn update_user(
    State(app): State<Arc<AuthApp>>,
    id: Path<Uuid>,
    payload: Json<UpdateUserRequest>,
) -> Result<impl IntoResponse, ApiError> {
    app.http_delivery.update_user(id, payload).await
}

pub async fn delete_user(
    State(app): State<Arc<AuthApp>>,
    payload: Path<Uuid>,
) -> Result<impl IntoResponse, ApiError> {
    app.http_delivery.delete_user(payload).await
}

pub async fn get_user(
    State(app): State<Arc<AuthApp>>,
    payload: Path<Uuid>,
) -> Result<impl IntoResponse, ApiError> {
    app.http_delivery.get_user(payload).await
}

pub async fn login(
    State(app): State<Arc<AuthApp>>,
    jar: CookieJar,
    payload: Json<LoginRequest>,
) -> Result<impl IntoResponse, ApiError> {
    app.http_delivery.login(jar, payload).await
}

pub async fn logout(
    State(app): State<Arc<AuthApp>>,
    jar: CookieJar,
) -> Result<impl IntoResponse, ApiError> {
    app.http_delivery.logout(jar).await
}

pub async fn get_user_from_cookie(
    State(app): State<Arc<AuthApp>>,
    jar: CookieJar,
) -> Result<impl IntoResponse, ApiError> {
    app.http_delivery.get_user_from_cookie(jar).await
}
