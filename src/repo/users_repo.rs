use crate::delivery_http::users_delivery::IUsersRepo;
use crate::errors::DBError;
use crate::errors::DBError::FailedToCreateUser;
use crate::infra::postgres::PGPool;
use crate::model::User;
use async_trait::async_trait;

pub struct UserRepo {
    pub repo: PGPool,
}

impl UserRepo {
    pub fn new(pool: PGPool) -> Self {
        UserRepo { repo: pool }
    }
}

#[async_trait]
impl IUsersRepo for UserRepo {
    async fn create_user(&self, user: User) -> Result<User, DBError> {
        let user = sqlx::query_as!(
            User,
            r#"insert into users (id, email, username, password_hash, salt)
            values ($1, $2, $3, $4, $5)
            returning id, email, username, password_hash, salt, created_at, updated_at;"#,
            user.id,
            user.email,
            user.username,
            user.password_hash,
            user.salt
        )
        .fetch_one(&self.repo.pool)
        .await
        .map_err(FailedToCreateUser)?;

        Ok(user)
    }
}
