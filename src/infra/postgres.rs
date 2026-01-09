use crate::errors::DBInfraError;
use crate::errors::DBInfraError::{
    FailedToAcquirePG, FailedToInitPGPool, FailedToPingPG, FailedToRunMigrations,
};
use sqlx::postgres::PgPoolOptions;
use sqlx::{Connection, Pool, Postgres};

pub struct PGPool {
    pub pool: Pool<Postgres>,
}

impl PGPool {
    pub async fn new(connection: String) -> Result<Self, DBInfraError> {
        let pool = PgPoolOptions::new()
            .max_connections(5)
            .connect(connection.as_str())
            .await
            .map_err(FailedToInitPGPool)?;

        // TODO: check if ping is required
        Self::ping(&pool).await?;

        sqlx::migrate!("./migrations")
            .run(&pool)
            .await
            .map_err(FailedToRunMigrations)?;

        Ok(PGPool { pool })
    }

    async fn ping(pool: &Pool<Postgres>) -> Result<(), DBInfraError> {
        match pool.acquire().await {
            Ok(mut conn) => {
                if let Err(e) = conn.ping().await {
                    return Err(e).map_err(FailedToPingPG);
                }
            }

            Err(e) => return Err(e).map_err(FailedToAcquirePG),
        }

        Ok(())
    }
}
