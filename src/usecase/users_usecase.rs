use crate::delivery_http::dto::{LoginRequest, RegisterRequest};
use crate::delivery_http::users_delivery::IUsersCreatorUsecase;
use crate::errors::UsecaseError::{DBDerivedError, InvalidCreds, UserNotFoundError};
use crate::errors::{DBError, UsecaseError};
use crate::model::User;
use argon2::password_hash::SaltString;
use argon2::password_hash::rand_core::OsRng;
use argon2::{Argon2, PasswordHash, PasswordHasher, PasswordVerifier};
use async_trait::async_trait;
use std::sync::Arc;
use uuid::Uuid;

#[async_trait]
pub trait IUsersRepository: Send + Sync {
    async fn create_user(&self, user: User) -> Result<User, DBError>;
    async fn login(&self, email: String) -> Result<Option<User>, DBError>;
}

pub struct UserUsecase {
    repo: Arc<dyn IUsersRepository>,
}

impl UserUsecase {
    pub fn new(repo: Arc<dyn IUsersRepository>) -> Self {
        UserUsecase { repo }
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

        Ok(self.repo.create_user(user).await.map_err(DBDerivedError)?)
    }

    async fn login(&self, login_payload: LoginRequest) -> Result<User, UsecaseError> {
        let user = self.repo.login(login_payload.email).await?;

        // TODO: mb change to is_none()
        let user = match user {
            Some(user) => user,
            None => {
                return Err(UserNotFoundError);
            }
        };

        let argon2 = Argon2::default();

        let parsed_hash = PasswordHash::new(user.password_hash.as_str())?;

        if argon2
            .verify_password(login_payload.password.as_bytes(), &parsed_hash)
            .is_ok()
        {
            Ok(user)
        } else {
            Err(InvalidCreds)
        }
    }
}
