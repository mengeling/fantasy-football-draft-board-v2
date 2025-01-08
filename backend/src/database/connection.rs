use sqlx::{Error, PgPool, Pool, Postgres};
use std::env;

pub async fn get_db_connection() -> Result<Pool<Postgres>, Error> {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let conn = PgPool::connect(&database_url).await?;
    Ok(conn)
}
