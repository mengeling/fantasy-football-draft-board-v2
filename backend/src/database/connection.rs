use sqlx::{Error, PgPool, Pool, Postgres};
use std::env;
use std::sync::OnceLock;

static POOL: OnceLock<PgPool> = OnceLock::new();

pub async fn init_pool() -> Result<(), Error> {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = PgPool::connect(&database_url).await?;
    POOL.set(pool).expect("Failed to set database pool");
    Ok(())
}

pub fn get_pool() -> Result<&'static PgPool, Error> {
    POOL.get().ok_or(Error::PoolClosed)
}

pub async fn get_db_connection() -> Result<Pool<Postgres>, Error> {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let conn = PgPool::connect(&database_url).await?;
    Ok(conn)
}
