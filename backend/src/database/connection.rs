use once_cell::sync::Lazy;
use sqlx::postgres::PgPoolOptions;
use sqlx::{Connection, Error, PgConnection, PgPool};
use std::env;

pub static DB_POOL: Lazy<PgPool> = Lazy::new(|| {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgPoolOptions::new()
        .max_connections(5)
        .connect_lazy(&database_url)
        .expect("Failed to create database pool")
});

pub async fn get_raw_connection() -> Result<PgConnection, Error> {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let conn = PgConnection::connect(&database_url).await?;
    Ok(conn)
}
