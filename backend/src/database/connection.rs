use sqlx::{Error, PgPool, Pool, Postgres};
use std::env;
use std::sync::OnceLock;

static POOL: OnceLock<PgPool> = OnceLock::new();

pub async fn init_pool() -> Result<(), Error> {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = PgPool::connect(&database_url).await?;
    ensure_schema(&pool).await?;
    POOL.set(pool).expect("Failed to set database pool");
    Ok(())
}

// Applies the idempotent schema on startup so a deploy that adds a table or
// column self-migrates — no manual step to remember per environment.
async fn ensure_schema(pool: &PgPool) -> Result<(), Error> {
    sqlx::raw_sql(include_str!("setup_db.sql"))
        .execute(pool)
        .await?;
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
