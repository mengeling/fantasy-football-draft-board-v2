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
            position TEXT NOT NULL,
            team TEXT NOT NULL,
            bye_week INTEGER,
            height TEXT NOT NULL,
            weight TEXT NOT NULL,
            age INTEGER,
            college TEXT NOT NULL          
        )",
    )
    .execute(&*DB_POOL)
    .await?;

    sqlx::query(
        "CREATE TABLE IF NOT EXISTS rankings (
            player_id INTEGER PRIMARY KEY REFERENCES players(id),
            overall INTEGER,
            position INTEGER
        )",
    )
    .execute(&*DB_POOL)
    .await?;

    sqlx::query(
        "CREATE TABLE IF NOT EXISTS stats (
            player_id INTEGER PRIMARY KEY REFERENCES players(id),
            pass_cmp DOUBLE PRECISION,
            pass_att DOUBLE PRECISION,
            pass_cmp_pct DOUBLE PRECISION,
            pass_yds DOUBLE PRECISION,
            pass_yds_per_att DOUBLE PRECISION,
            pass_td DOUBLE PRECISION,
            pass_int DOUBLE PRECISION,
            pass_sacks DOUBLE PRECISION,
            rush_att DOUBLE PRECISION,
            rush_yds DOUBLE PRECISION,
            rush_yds_per_att DOUBLE PRECISION,
            rush_long DOUBLE PRECISION,
            rush_20 DOUBLE PRECISION,
            rush_td DOUBLE PRECISION,
            fumbles DOUBLE PRECISION,
            receptions DOUBLE PRECISION,
            rec_tgt DOUBLE PRECISION,
            rec_yds DOUBLE PRECISION,
            rec_yds_per_rec DOUBLE PRECISION,
            rec_long DOUBLE PRECISION,
            rec_20 DOUBLE PRECISION,
            rec_td DOUBLE PRECISION,
            field_goals DOUBLE PRECISION,
            fg_att DOUBLE PRECISION,
            fg_pct DOUBLE PRECISION,
            fg_long DOUBLE PRECISION,
            fg_1_19 DOUBLE PRECISION,
            fg_20_29 DOUBLE PRECISION,
            fg_30_39 DOUBLE PRECISION,
            fg_40_49 DOUBLE PRECISION,
            fg_50 DOUBLE PRECISION,
            extra_points DOUBLE PRECISION,
            xp_att DOUBLE PRECISION,
            sacks DOUBLE PRECISION,
            int DOUBLE PRECISION,
            fumbles_recovered DOUBLE PRECISION,
            fumbles_forced DOUBLE PRECISION,
            def_td DOUBLE PRECISION,
            safeties DOUBLE PRECISION,
            special_teams_td DOUBLE PRECISION,
            games DOUBLE PRECISION,
            fantasy_pts DOUBLE PRECISION,
            fantasy_pts_per_game DOUBLE PRECISION
        )",
    )
    .execute(&*DB_POOL)
    .await?;

    Ok(())
}
