use crate::app::IUsersDelivery;
use crate::delivery_http::dto::{
    RegisterRequest, UpdateUserRequest, UserNotFoundResponse, UserResponse,
};
use crate::errors::ApiError::UseCaseError;
use crate::errors::{ApiError, DBError, UsecaseError};
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
    async fn update_user(&self, user: User) -> Result<Option<User>, DBError>;
    async fn get_user(&self, user_id: Uuid) -> Result<Option<User>, DBError>;
    async fn delete_user(&self, user_id: Uuid) -> Result<bool, DBError>;
}

#[async_trait]
pub trait IUsersCreatorUsecase: Send + Sync {
    async fn create_user(&self, user_payload: RegisterRequest) -> Result<User, UsecaseError>;
}

pub struct UsersDelivery {
    repo: Arc<dyn IUsersRepo>,
    usecase: Arc<dyn IUsersCreatorUsecase>,
}

impl UsersDelivery {
    pub fn new(repo: Arc<dyn IUsersRepo>, usecase: Arc<dyn IUsersCreatorUsecase>) -> Self {
        UsersDelivery { repo, usecase }
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
        let user = self
            .usecase
            .create_user(payload)
            .await
            .map_err(UseCaseError)?;

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
