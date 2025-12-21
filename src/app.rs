use crate::delivery_http::dto::RegisterRequest;
use crate::errors::ApiError;
use async_trait::async_trait;
use axum::Json;
use axum::response::Response;
use std::sync::Arc;

#[async_trait]
pub trait IUsersDelivery: Send + Sync {
    async fn create_user(&self, payload: Json<RegisterRequest>) -> Result<Response, ApiError>;
}

pub struct AuthApp {
    pub(crate) delivery: Arc<dyn IUsersDelivery>,
}

impl AuthApp {
    pub async fn new(delivery: Arc<dyn IUsersDelivery>) -> Self {
        AuthApp { delivery }
    }
}
