use once_cell::sync::Lazy;
use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;
use std::env;
use strum::IntoEnumIterator;

use crate::models::{
    player::{Position, Team},
    rankings::ScoringSettings,
};

pub static DB_POOL: Lazy<PgPool> = Lazy::new(|| {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgPoolOptions::new()
        .max_connections(5)
        .connect_lazy(&database_url)
        .expect("Failed to create database pool")
});

pub async fn init_db() -> Result<(), sqlx::Error> {
    sqlx::query("DROP TABLE IF EXISTS rankings")
        .execute(&*DB_POOL)
        .await?;

    sqlx::query("DROP TABLE IF EXISTS stats")
        .execute(&*DB_POOL)
        .await?;

    sqlx::query("DROP TABLE IF EXISTS players CASCADE")
        .execute(&*DB_POOL)
        .await?;

    sqlx::query("DROP TABLE IF EXISTS drafted_players")
        .execute(&*DB_POOL)
        .await?;

    sqlx::query("DROP TABLE IF EXISTS users")
        .execute(&*DB_POOL)
        .await?;

    let position_variants: String = Position::iter()
        .map(|v| format!("'{}'", v.to_string()))
        .collect::<Vec<_>>()
        .join(", ");

    let team_variants: String = Team::iter()
        .map(|v| format!("'{}'", v.to_string()))
        .collect::<Vec<_>>()
        .join(", ");

    let scoring_variants: String = ScoringSettings::iter()
        .map(|v| format!("'{}'", v.to_string()))
        .collect::<Vec<_>>()
        .join(", ");

    sqlx::query(&format!(
        "DROP TYPE IF EXISTS POSITION CASCADE;
         DROP TYPE IF EXISTS TEAM CASCADE;
         DROP TYPE IF EXISTS SCORING_SETTINGS CASCADE;
         
         CREATE TYPE POSITION AS ENUM ({position_variants});
         CREATE TYPE TEAM AS ENUM ({team_variants});
         CREATE TYPE SCORING_SETTINGS AS ENUM ({scoring_variants});"
    ))
    .execute(&*DB_POOL)
    .await?;

    sqlx::query(
        "CREATE TABLE IF NOT EXISTS players (
            id INTEGER PRIMARY KEY,
            name TEXT NOT NULL,
            position POSITION NOT NULL,
            team TEAM NOT NULL,
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
            player_id INTEGER,
            scoring_settings SCORING_SETTINGS,
            overall INTEGER,
            position INTEGER,
            PRIMARY KEY (player_id, scoring_settings)
        )",
    )
    .execute(&*DB_POOL)
    .await?;

    sqlx::query(
        "CREATE TABLE IF NOT EXISTS stats (
            player_id INTEGER PRIMARY KEY,
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
            standard_pts DOUBLE PRECISION,
            standard_pts_per_game DOUBLE PRECISION,
            half_ppr_pts DOUBLE PRECISION,
            half_ppr_pts_per_game DOUBLE PRECISION,
            ppr_pts DOUBLE PRECISION,
            ppr_pts_per_game DOUBLE PRECISION
        )",
    )
    .execute(&*DB_POOL)
    .await?;

    sqlx::query(
        "CREATE TABLE IF NOT EXISTS users (
            id SERIAL PRIMARY KEY,
            username VARCHAR(255) NOT NULL UNIQUE,
            scoring_settings SCORING_SETTINGS,
            created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP NOT NULL
        )",
    )
    .execute(&*DB_POOL)
    .await?;

    sqlx::query(
        "CREATE TABLE IF NOT EXISTS drafted_players (
            id SERIAL PRIMARY KEY,
            user_id INTEGER NOT NULL REFERENCES users(id) ON DELETE CASCADE,
            player_id INTEGER NOT NULL REFERENCES players(id) ON DELETE CASCADE,
            drafted_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP NOT NULL,
            UNIQUE(user_id, player_id)
        )",
    )
    .execute(&*DB_POOL)
    .await?;

    Ok(())
}
