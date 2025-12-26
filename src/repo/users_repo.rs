use crate::delivery_http::users_delivery::IUsersRepo;
use crate::errors::DBError;
use crate::errors::DBError::{
    FailedToCreateUser, FailedToDeleteUser, FailedToGetUser, FailedToUpdateUser,
};
use crate::infra::postgres::PGPool;
use crate::model::User;
use crate::usecase::users_usecase::IUsersCreatorRepo;
use async_trait::async_trait;
use uuid::Uuid;

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
    async fn update_user(&self, user: User) -> Result<Option<User>, DBError> {
        let user = sqlx::query_as!(
            User,
            r"update users set username = $1 where id = $2
            returning id, email, username, password_hash, created_at, updated_at;",
            user.username,
            user.id,
        )
        .fetch_optional(&self.repo.pool)
        .await
        .map_err(FailedToUpdateUser)?;

        Ok(user)
    }

    async fn get_user(&self, user_id: Uuid) -> Result<Option<User>, DBError> {
        let user = sqlx::query_as!(
            User,
            r"select id, email, username, password_hash, created_at, updated_at
            from users
            where id = $1;",
            user_id,
        )
        .fetch_optional(&self.repo.pool)
        .await
        .map_err(FailedToGetUser)?;

        Ok(user)
    }

    async fn delete_user(&self, user_id: Uuid) -> Result<bool, DBError> {
        let res = sqlx::query!(r"delete from users where id = $1;", user_id)
            .execute(&self.repo.pool)
            .await
            .map_err(FailedToDeleteUser)?;

        Ok(res.rows_affected() == 1)
    }
}

#[async_trait]
impl IUsersCreatorRepo for UserRepo {
    async fn create_user(&self, user: User) -> Result<User, DBError> {
        let user = sqlx::query_as!(
            User,
            r#"insert into users (id, email, username, password_hash)
            values ($1, $2, $3, $4)
            returning id, email, username, password_hash, created_at, updated_at;"#,
            user.id,
            user.email,
            user.username,
            user.password_hash,
        )
        .fetch_one(&self.repo.pool)
        .await
        .map_err(FailedToCreateUser)?;

        Ok(user)
    }
}
