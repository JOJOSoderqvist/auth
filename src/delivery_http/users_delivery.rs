use crate::app::IUsersDelivery;
use crate::delivery_http::dto::{
    RegisterRequest, UpdateUserRequest, UserNotFoundResponse, UserResponse,
};
use crate::errors::ApiError::DataBaseError;
use crate::errors::{ApiError, DBError};
use crate::model::User;
use async_trait::async_trait;
use axum::Json;
use axum::extract::Path;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use std::sync::Arc;
use uuid::Uuid;

#[async_trait]
pub trait IUsersRepo: Send + Sync {
    async fn create_user(&self, user: User) -> Result<User, DBError>;
    async fn update_user(&self, user: User) -> Result<Option<User>, DBError>;
    async fn get_user(&self, user_id: Uuid) -> Result<Option<User>, DBError>;
    async fn delete_user(&self, user_id: Uuid) -> Result<bool, DBError>;
}

pub struct UsersDelivery {
    repo: Arc<dyn IUsersRepo>,
}

impl UsersDelivery {
    pub fn new(repo: Arc<dyn IUsersRepo>) -> Self {
        UsersDelivery { repo }
    }

    fn respond_with_user(user: Option<User>) -> Result<Response, ApiError> {
        if let Some(user) = user {
            Ok((StatusCode::OK, Json::<UserResponse>(user.into())).into_response())
        } else {
            Ok((
                StatusCode::NOT_FOUND,
                Json::<UserNotFoundResponse>(UserNotFoundResponse::default()),
            )
                .into_response())
        }
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
            created_at: Default::default(),
            updated_at: Default::default(),
        };

        let user = self
            .repo
            .create_user(mock_user)
            .await
            .map_err(DataBaseError)?;

        Ok((StatusCode::CREATED, Json::<UserResponse>(user.into())).into_response())
    }

    async fn get_user(&self, Path(payload): Path<Uuid>) -> Result<Response, ApiError> {
        let user = self.repo.get_user(payload).await?;
        Self::respond_with_user(user)
    }

    async fn update_user(
        &self,
        Path(id): Path<Uuid>,
        Json(payload): Json<UpdateUserRequest>,
    ) -> Result<Response, ApiError> {
        let mut update_user_req: User = payload.into();
        update_user_req.id = id;

        let user = self.repo.update_user(update_user_req).await?;
        Self::respond_with_user(user)
    }

    async fn delete_user(&self, Path(payload): Path<Uuid>) -> Result<Response, ApiError> {
        let is_deleted = self.repo.delete_user(payload).await?;
        if is_deleted {
            Ok(StatusCode::NO_CONTENT.into_response())
        } else {
            Ok((
                StatusCode::NOT_FOUND,
                Json::<UserNotFoundResponse>(UserNotFoundResponse::default()),
            )
                .into_response())
        }
    }
}
