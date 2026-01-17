use crate::delivery_http::users_delivery::IUsersRepo;
use crate::errors::DBError;
use crate::errors::DBError::{
    FailedToCreateUser, FailedToDeleteUser, FailedToGetUser, FailedToUpdateUser,
};
use crate::infra::postgres::PGPool;
use crate::model::User;
use crate::usecase::users_usecase::IUsersRepository;
use async_trait::async_trait;
use uuid::Uuid;

pub struct UsersRepo {
    pub repo: PGPool,
}

impl UsersRepo {
    pub fn new(pool: PGPool) -> Self {
        UsersRepo { repo: pool }
    }
}

#[async_trait]
impl IUsersRepo for UsersRepo {
    async fn update_user(&self, user: User) -> Result<Option<User>, DBError> {
        let user = sqlx::query_as(
            r"update users set username = $1 where id = $2
            returning id, email, username, password_hash, created_at, updated_at;",
        )
        .bind(user.username)
        .bind(user.id)
        .fetch_optional(&self.repo.pool)
        .await
        .map_err(FailedToUpdateUser)?;

        Ok(user)
    }

    async fn get_user(&self, user_id: Uuid) -> Result<Option<User>, DBError> {
        let user = sqlx::query_as(
            r"select id, email, username, password_hash, created_at, updated_at
            from users
            where id = $1;",
        )
        .bind(user_id)
        .fetch_optional(&self.repo.pool)
        .await
        .map_err(FailedToGetUser)?;

        Ok(user)
    }

    async fn delete_user(&self, user_id: Uuid) -> Result<bool, DBError> {
        let res = sqlx::query(r"delete from users where id = $1;")
            .bind(user_id)
            .execute(&self.repo.pool)
            .await
            .map_err(FailedToDeleteUser)?;

        Ok(res.rows_affected() == 1)
    }
}

#[async_trait]
impl IUsersRepository for UsersRepo {
    async fn create_user(&self, user: User) -> Result<User, DBError> {
        let res = sqlx::query_as(
            r#"insert into users (id, email, username, password_hash)
            values ($1, $2, $3, $4)
            returning id, email, username, password_hash, created_at, updated_at;"#,
        )
        .bind(user.id)
        .bind(user.email)
        .bind(user.username)
        .bind(user.password_hash)
        .fetch_one(&self.repo.pool)
        .await;

        match res {
            Ok(user) => Ok(user),
            Err(e) => {
                if let Some(db_err) = e.as_database_error() {
                    // Unique violation check
                    if let Some(code) = db_err.code() {
                        if code == "23505" {
                            return Err(DBError::UserAlreadyExists);
                        }
                    }
                }

                Err(FailedToCreateUser(e))
            }
        }
    }

    async fn login(&self, email: String) -> Result<Option<User>, DBError> {
        let user = sqlx::query_as(
            r"select id, email, username, password_hash, created_at, updated_at
            from users
            where email = $1;",
        )
        .bind(email)
        .fetch_optional(&self.repo.pool)
        .await
        .map_err(FailedToGetUser)?;

        Ok(user)
    }
}
