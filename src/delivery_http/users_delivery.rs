use crate::app::IUsersDelivery;
use crate::delivery_http::dto::{
    LoginRequest, RegisterRequest, UpdateUserRequest, UserNotFoundResponse, UserResponse,
};
use crate::errors::ApiError::UseCaseError;
use crate::errors::{ApiError, DBError, UsecaseError};
use crate::model::User;
use async_trait::async_trait;
use axum::Json;
use axum::extract::Path;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};

use axum_extra::extract::cookie::{Cookie, CookieJar};

use crate::errors::DBError::FailedToParseUUID;
use std::sync::Arc;
use uuid::Uuid;
use crate::delivery_grpc::users_delivery::IUserIDGetter;

#[async_trait]
pub trait IUsersRepo: Send + Sync {
    async fn update_user(&self, user: User) -> Result<Option<User>, DBError>;
    async fn get_user(&self, user_id: Uuid) -> Result<Option<User>, DBError>;
    async fn delete_user(&self, user_id: Uuid) -> Result<bool, DBError>;
}

#[async_trait]
pub trait IUsersCreatorUsecase: Send + Sync {
    async fn create_user(&self, user_payload: RegisterRequest) -> Result<User, UsecaseError>;
    async fn login(&self, login_payload: LoginRequest) -> Result<User, UsecaseError>;
}

#[async_trait]
pub trait ISessionStore: Send + Sync {
    async fn create_session(&self, user_id: Uuid) -> Result<Uuid, DBError>;
    async fn remove_session(&self, session_id: Uuid) -> Result<(), DBError>;
}

pub struct UsersDelivery {
    repo: Arc<dyn IUsersRepo>,
    usecase: Arc<dyn IUsersCreatorUsecase>,
    session_store: Arc<dyn ISessionStore>,
    user_id_getter: Arc<dyn IUserIDGetter>,
}

impl UsersDelivery {
    pub fn new(
        repo: Arc<dyn IUsersRepo>,
        usecase: Arc<dyn IUsersCreatorUsecase>,
        session_store: Arc<dyn ISessionStore>,
        user_id_getter: Arc<dyn IUserIDGetter>,
    ) -> Self {
        UsersDelivery {
            repo,
            usecase,
            session_store,
            user_id_getter
        }
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

    fn create_auth_cookie(session_id: Uuid) -> Cookie<'static> {
        Cookie::build(("session_id", session_id.to_string()))
            .path("/")
            .http_only(true)
            .secure(false)
            .build()
    }
}

#[async_trait]
impl IUsersDelivery for UsersDelivery {
    async fn create_user(
        &self,
        jar: CookieJar,
        Json(payload): Json<RegisterRequest>,
    ) -> Result<Response, ApiError> {
        let user = self
            .usecase
            .create_user(payload)
            .await
            .map_err(UseCaseError)?;

        let session_id = self.session_store.create_session(user.id).await?;

        let cookie = Self::create_auth_cookie(session_id);

        Ok((
            StatusCode::CREATED,
            jar.add(cookie),
            Json::<UserResponse>(user.into()),
        )
            .into_response())
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

    async fn login(
        &self,
        jar: CookieJar,
        Json(payload): Json<LoginRequest>,
    ) -> Result<Response, ApiError> {
        let user = self.usecase.login(payload).await?;

        let session_id = self.session_store.create_session(user.id).await?;

        let cookie = Self::create_auth_cookie(session_id);

        Ok((
            StatusCode::OK,
            jar.add(cookie),
            Json::<UserResponse>(user.into()),
        )
            .into_response())
    }

    async fn logout(&self, jar: CookieJar) -> Result<Response, ApiError> {
        if let Some(cookie) = jar.get("session_id") {
            let session_id = cookie.value().to_string();

            let session_id = Uuid::parse_str(session_id.as_str()).map_err(FailedToParseUUID)?;

            self.session_store.remove_session(session_id).await?;
        }

        let removal_cookie = Cookie::build("session_id").path("/").build();
        Ok((StatusCode::OK, jar.remove(removal_cookie)).into_response())
    }

    async fn get_user_from_cookie(&self, jar: CookieJar) -> Result<Response, ApiError> {
        if let Some(cookie) = jar.get("session_id") {
            let session_id = cookie.value().to_string();

            let session_id = Uuid::parse_str(session_id.as_str()).map_err(FailedToParseUUID)?;

            if let Some(user_id) = self.user_id_getter.get_user(session_id).await? {
                if let Some(user) = self.repo.get_user(user_id).await? {
                    return Self::respond_with_user(Some(user));
                }
                return Self::respond_with_user(None);
            }
        }

        Ok((
            StatusCode::UNAUTHORIZED,
        ).into_response())
    }
}
