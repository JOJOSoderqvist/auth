use crate::errors::DBError::FailedToCreateSession;
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
}
