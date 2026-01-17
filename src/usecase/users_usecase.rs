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
use mockall::predicate::*;

#[cfg_attr(test, mockall::automock)]
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::errors::DBError;
    use std::sync::Arc;
    use uuid::Uuid;

    fn mock_user() -> User {
        User {
            id: Uuid::new_v4(),
            email: "test@example.com".to_string(),
            username: "testuser".to_string(),
            password_hash: "argon2_hash_placeholder".to_string(),
            created_at: Default::default(),
            updated_at: Default::default(),
        }
    }

    #[tokio::test]
    async fn test_create_user_success() {
        let mut mock_repo = MockIUsersRepository::new();

        mock_repo
            .expect_create_user()
            .times(1)
            .withf(|u: &User| u.email == "new@email.com" && u.username == "NewUser")
            .returning(|u| Ok(u));

        let usecase = UserUsecase::new(Arc::new(mock_repo));

        let req = RegisterRequest {
            email: "new@email.com".to_string(),
            username: "NewUser".to_string(),
            password: "password123".to_string(),
        };

        let result = usecase.create_user(req).await;

        assert!(result.is_ok());
        let user = result.unwrap();
        assert_eq!(user.email, "new@email.com");
        assert_ne!(user.password_hash, "password123");
        assert!(!user.password_hash.is_empty());
    }

    #[tokio::test]
    async fn test_create_user_duplicate_error() {
        let mut mock_repo = MockIUsersRepository::new();

        mock_repo
            .expect_create_user()
            .times(1)
            .returning(|_| Err(DBError::UserAlreadyExists));

        let usecase = UserUsecase::new(Arc::new(mock_repo));

        let req = RegisterRequest {
            email: "exists@email.com".to_string(),
            username: "User".to_string(),
            password: "pwd".to_string(),
        };

        let result = usecase.create_user(req).await;

        assert!(result.is_err());
        match result.unwrap_err() {
            DBDerivedError(_) => assert!(true),
            _ => assert!(false, "Wrong error type"),
        }
    }

    #[tokio::test]
    async fn test_login_success() {
        let mut mock_repo = MockIUsersRepository::new();


        let password = "mysecretpassword";
        let salt = SaltString::generate(&mut OsRng);
        let argon2 = Argon2::default();
        let valid_hash = argon2.hash_password(password.as_bytes(), &salt).unwrap().to_string();

        let mut db_user = mock_user();
        db_user.email = "login@test.com".to_string();
        db_user.password_hash = valid_hash;

        mock_repo
            .expect_login()
            .times(1)
            .with(eq("login@test.com".to_string()))
            .return_once(move |_| Ok(Some(db_user)));

        let usecase = UserUsecase::new(Arc::new(mock_repo));

        let req = LoginRequest {
            email: "login@test.com".to_string(),
            password: "mysecretpassword".to_string(),
        };

        let result = usecase.login(req).await;

        assert!(result.is_ok());
        assert_eq!(result.unwrap().email, "login@test.com");
    }

    #[tokio::test]
    async fn test_login_wrong_password() {
        let mut mock_repo = MockIUsersRepository::new();

        let salt = SaltString::generate(&mut OsRng);
        let valid_hash = Argon2::default()
            .hash_password("correct_password".as_bytes(), &salt)
            .unwrap()
            .to_string();

        let mut db_user = mock_user();
        db_user.password_hash = valid_hash;

        mock_repo
            .expect_login()
            .times(1)
            .returning(move |_| Ok(Some(db_user.clone())));

        let usecase = UserUsecase::new(Arc::new(mock_repo));

        let req = LoginRequest {
            email: "test@test.com".to_string(),
            password: "WRONG_PASSWORD".to_string(),
        };

        let result = usecase.login(req).await;

        assert!(matches!(result, Err(InvalidCreds)));
    }

    #[tokio::test]
    async fn test_login_user_not_found() {
        let mut mock_repo = MockIUsersRepository::new();

        mock_repo
            .expect_login()
            .times(1)
            .returning(|_| Ok(None));

        let usecase = UserUsecase::new(Arc::new(mock_repo));

        let req = LoginRequest {
            email: "unknown@test.com".to_string(),
            password: "123".to_string(),
        };

        let result = usecase.login(req).await;

        assert!(matches!(result, Err(UserNotFoundError)));
    }
}