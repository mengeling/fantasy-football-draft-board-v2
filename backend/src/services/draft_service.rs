use crate::models::drafted_player::DraftedPlayer;
use crate::models::player::PlayerDenormalized;
use sqlx::{Error, PgPool};

pub async fn draft_player(
    pool: &PgPool,
    user_id: i32,
    player_id: i32,
) -> Result<DraftedPlayer, sqlx::Error> {
    sqlx::query_as!(
        DraftedPlayer,
        r#"
        INSERT INTO drafted_players (user_id, player_id)
        VALUES ($1, $2)
        RETURNING id, user_id, player_id, drafted_at
        "#,
        user_id,
        player_id
    )
    .fetch_one(pool)
    .await
}

pub async fn undraft_player(
    pool: &PgPool,
    user_id: i32,
    player_id: i32,
) -> Result<bool, sqlx::Error> {
    let result = sqlx::query!(
        r#"
        DELETE FROM drafted_players
        WHERE user_id = $1 AND player_id = $2
        "#,
        user_id,
        player_id
    )
    .execute(pool)
    .await?;

    Ok(result.rows_affected() > 0)
}

pub async fn get_player_data(pool: &PgPool, player_id: i32) -> Result<PlayerDenormalized, Error> {
    let player_denormalized = sqlx::query_as!(
        PlayerDenormalized,
        r#"
        SELECT 
            p.id,
            p.name,
            p.position,
            p.team,
            p.bye_week,
            p.height,
            p.weight,
            p.age,
            p.college,
            r.overall as overall_ranking,
            r.position as position_ranking,
            s.pass_cmp,
            s.pass_att,
            s.pass_cmp_pct,
            s.pass_yds,
            s.pass_yds_per_att,
            s.pass_td,
            s.pass_int,
            s.pass_sacks,
            s.rush_att,
            s.rush_yds,
            s.rush_yds_per_att,
            s.rush_long,
            s.rush_20,
            s.rush_td,
            s.fumbles,
            s.receptions,
            s.rec_tgt,
            s.rec_yds,
            s.rec_yds_per_rec,
            s.rec_long,
            s.rec_20,
            s.rec_td,
            s.field_goals,
            s.fg_att,
            s.fg_pct,
            s.fg_long,
            s.fg_1_19,
            s.fg_20_29,
            s.fg_30_39,
            s.fg_40_49,
            s.fg_50,
            s.extra_points,
            s.xp_att,
            s.sacks,
            s.int,
            s.fumbles_recovered,
            s.fumbles_forced,
            s.def_td,
            s.safeties,
            s.special_teams_td,
            s.games,
            s.standard_pts,
            s.standard_pts_per_game,
            s.half_ppr_pts,
            s.half_ppr_pts_per_game,
            s.ppr_pts,
            s.ppr_pts_per_game
        FROM players p
        LEFT JOIN rankings r ON p.id = r.player_id
        LEFT JOIN stats s ON p.id = s.player_id
        WHERE p.id = $1
        "#,
        player_id
    )
    .fetch_optional(pool)
    .await?
    .ok_or_else(|| Error::NotFound(format!("Player not found with id: {}", player_id)))?;

    Ok(player_denormalized)
}