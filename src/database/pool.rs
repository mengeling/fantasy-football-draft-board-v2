use once_cell::sync::Lazy;
use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;
use std::env;

pub static DB_POOL: Lazy<PgPool> = Lazy::new(|| {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgPoolOptions::new()
        .max_connections(5)
        .connect_lazy(&database_url)
        .expect("Failed to create database pool")
});

pub async fn init_db() -> Result<(), sqlx::Error> {
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS players (
            id INTEGER PRIMARY KEY,
            name TEXT NOT NULL,
            team TEXT NOT NULL,
            position TEXT NOT NULL,
            overall_ranking INTEGER,
            position_ranking INTEGER,
            bye_week INTEGER,
            bio_url TEXT NOT NULL
        )",
    )
    .execute(&*DB_POOL)
    .await?;

    sqlx::query(
        "CREATE TABLE IF NOT EXISTS player_bios (
            player_id INTEGER PRIMARY KEY REFERENCES players(id),
            image_url TEXT NOT NULL,
            height TEXT NOT NULL,
            weight TEXT NOT NULL,
            age INTEGER,
            college TEXT NOT NULL
        )",
    )
    .execute(&*DB_POOL)
    .await?;

    sqlx::query(
        "CREATE TABLE IF NOT EXISTS player_stats (
            player_id INTEGER REFERENCES players(id),
            stat_name TEXT NOT NULL,
            stat_value DOUBLE PRECISION,
            PRIMARY KEY (player_id, stat_name)
        )",
    )
    .execute(&*DB_POOL)
    .await?;

    Ok(())
}
