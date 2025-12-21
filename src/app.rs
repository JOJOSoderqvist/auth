use crate::delivery_http::dto::{RegisterRequest, UpdateUserRequest};
use crate::errors::ApiError;
use async_trait::async_trait;
use axum::Json;
use axum::extract::Path;
use axum::response::Response;
use std::sync::Arc;
use uuid::Uuid;

#[async_trait]
pub trait IUsersDelivery: Send + Sync {
    async fn create_user(&self, payload: Json<RegisterRequest>) -> Result<Response, ApiError>;
    async fn get_user(&self, payload: Path<Uuid>) -> Result<Response, ApiError>;
    async fn update_user(
        &self,
        id: Path<Uuid>,
        payload: Json<UpdateUserRequest>,
    ) -> Result<Response, ApiError>;
    async fn delete_user(&self, payload: Path<Uuid>) -> Result<Response, ApiError>;
}

pub struct AuthApp {
    pub delivery: Arc<dyn IUsersDelivery>,
}

impl AuthApp {
    pub async fn new(delivery: Arc<dyn IUsersDelivery>) -> Self {
        AuthApp { delivery }
    }
}
