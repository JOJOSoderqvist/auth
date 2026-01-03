use crate::errors::DBError::{
    FailedToCreateSession, FailedToDeleteSession, FailedToGetUserFromSession, FailedToParseUUID,
    SessionNotFound,
};
use async_trait::async_trait;
use deadpool_redis::redis::AsyncTypedCommands;
use uuid::Uuid;

const DEFAULT_EXPIRATION_TIME: u64 = 86400;

use crate::{
    delivery_http::users_delivery::ISessionStore, errors::DBError, infra::redis::RedisPool,
};

pub struct SessionsRepo {
    pub repo: RedisPool,
}

impl SessionsRepo {
    pub fn new(repo: RedisPool) -> Self {
        SessionsRepo { repo }
    }
}

#[async_trait]
impl ISessionStore for SessionsRepo {
    async fn create_session(&self, user_id: Uuid) -> Result<Uuid, DBError> {
        let session_id = Uuid::new_v4();
        let mut conn = self.repo.get_conn().await?;

        conn.set_ex(
            session_id.to_string(),
            user_id.to_string(),
            DEFAULT_EXPIRATION_TIME,
        )
        .await
        .map_err(FailedToCreateSession)?;

        Ok(session_id)
    }

    async fn get_user(&self, session_id: Uuid) -> Result<Option<Uuid>, DBError> {
        let mut conn = self.repo.get_conn().await?;

        let user_id = conn
            .get(session_id.to_string())
            .await
            .map_err(FailedToGetUserFromSession)?;

        user_id
            .map(|id| Uuid::parse_str(id.as_str()).map_err(FailedToParseUUID))
            .transpose()
    }

    async fn remove_session(&self, session_id: Uuid) -> Result<(), DBError> {
        let mut conn = self.repo.get_conn().await?;

        let rows_deleted = conn
            .del(session_id.to_string())
            .await
            .map_err(FailedToDeleteSession)?;

        if rows_deleted == 0 {
            return Err(SessionNotFound);
        }

        Ok(())
    }
}
