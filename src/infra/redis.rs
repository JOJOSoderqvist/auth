use crate::errors::{DBError, DBInfraError};
use deadpool_redis::{Config, Connection, Pool, Runtime};

pub struct RedisPool {
    pool: Pool,
}

impl RedisPool {
    pub fn new(connection: String) -> Result<Self, DBInfraError> {
        let cfg = Config::from_url(connection);
        let pool = cfg.create_pool(Some(Runtime::Tokio1))?;
        Ok(RedisPool { pool })
    }

    pub async fn get_conn(&self) -> Result<Connection, DBError> {
        Ok(self.pool.get().await?)
    }
}
