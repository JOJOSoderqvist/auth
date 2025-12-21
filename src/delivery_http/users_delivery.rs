use crate::app::IUsersDelivery;
use crate::delivery_http::dto::{RegisterRequest, RegisterResponse};
use crate::errors::ApiError::DataBaseError;
use crate::errors::{ApiError, DBError};
use crate::model::User;
use async_trait::async_trait;
use axum::Json;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use std::sync::Arc;
use uuid::Uuid;

#[async_trait]
pub trait IUsersRepo: Send + Sync {
    async fn create_user(&self, user: User) -> Result<User, DBError>;
}

pub struct UsersDelivery {
    repo: Arc<dyn IUsersRepo>,
}

impl UsersDelivery {
    pub fn new(repo: Arc<dyn IUsersRepo>) -> Self {
        UsersDelivery { repo }
    }
}

#[async_trait]
impl IUsersDelivery for UsersDelivery {
    async fn create_user(
        &self,
        Json(payload): Json<RegisterRequest>,
    ) -> Result<Response, ApiError> {
        let mock_user = User {
            id: Uuid::new_v4(),
            email: payload.email,
            username: payload.username,
            password_hash: payload.password,
            salt: "xdd".to_string(),
            created_at: Default::default(),
            updated_at: Default::default(),
        };

        let user = self
            .repo
            .create_user(mock_user)
            .await
            .map_err(DataBaseError)?;

        Ok((StatusCode::CREATED, Json::<RegisterResponse>(user.into())).into_response())
    }
}
