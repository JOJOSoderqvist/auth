use crate::delivery_http::dto::RegisterRequest;
use crate::delivery_http::users_delivery::IUsersCreatorUsecase;
use crate::errors::UsecaseError::DBDerivedError;
use crate::errors::{DBError, UsecaseError};
use crate::model::User;
use argon2::password_hash::SaltString;
use argon2::password_hash::rand_core::OsRng;
use argon2::{Argon2, PasswordHash, PasswordHasher};
use async_trait::async_trait;
use std::sync::Arc;
use uuid::Uuid;

#[async_trait]
pub trait IUsersCreatorRepo: Send + Sync {
    async fn create_user(&self, user: User) -> Result<User, DBError>;
}

pub struct UserUsecase {
    creator: Arc<dyn IUsersCreatorRepo>,
}

impl UserUsecase {
    pub fn new(creator: Arc<dyn IUsersCreatorRepo>) -> Self {
        UserUsecase { creator }
    }
}

#[async_trait]
impl IUsersCreatorUsecase for UserUsecase {
    async fn create_user(&self, user_payload: RegisterRequest) -> Result<User, UsecaseError> {
        let salt = SaltString::generate(&mut OsRng);
        let argon2 = Argon2::default();
        let password_hash = argon2
            .hash_password(user_payload.password.as_bytes(), &salt)?
            .to_string();

        let password_hash = PasswordHash::new(&password_hash)?;

        let user = User {
            id: Uuid::new_v4(),
            email: user_payload.email,
            username: user_payload.username,
            password_hash: password_hash.to_string(),
            created_at: Default::default(),
            updated_at: Default::default(),
        };

        Ok(self
            .creator
            .create_user(user)
            .await
            .map_err(DBDerivedError)?)
    }
}
