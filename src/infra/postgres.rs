use sqlx::{PgPool, Pool, Postgres};
use sqlx::postgres::PgPoolOptions;
use crate::errors::DBError;
use crate::errors::DBError::FailedToInitPGPool;

pub struct PGPool {
    pool: Pool<Postgres>
}

impl PGPool {
    async fn new(connection: String) -> Result<Self, DBError> {
        let pool = PgPoolOptions::new()
            .max_connections(5)
            .connect(connection.as_str())
            .await
            .map_err(FailedToInitPGPool)?;

        Ok(PGPool{pool})
    }
}