use crate::database::connection::get_pool;
use crate::models::drafted_player::DraftedPlayer;
use crate::models::player::PlayerDenormalized;
use crate::models::player::Position;
use crate::models::player::Team;
use crate::models::rankings::ScoringSettings;
use crate::models::user::User;
use sqlx::Error;

pub async fn draft_player(user_id: i32, player_id: i32) -> Result<DraftedPlayer, sqlx::Error> {
    let pool = get_pool()?;
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

pub async fn undraft_player(user_id: i32, player_id: i32) -> Result<bool, sqlx::Error> {
    let pool = get_pool()?;
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

pub async fn get_user(username: &str) -> Result<Option<User>, sqlx::Error> {
    let pool = get_pool()?;
    sqlx::query_as!(
        User,
        r#"
        SELECT 
            id,
            username,
            scoring_settings as "scoring_settings!: ScoringSettings",
            created_at
        FROM users 
        WHERE username = $1
        "#,
        username
    )
    .fetch_optional(pool)
    .await
}

pub async fn create_user(
    username: &str,
    scoring_settings: &ScoringSettings,
) -> Result<User, sqlx::Error> {
    let pool = get_pool()?;
    sqlx::query_as!(
        User,
        r#"
        INSERT INTO users (username, scoring_settings)
        VALUES ($1, $2)
        RETURNING id, username, scoring_settings as "scoring_settings!: ScoringSettings", created_at
        "#,
        username,
        scoring_settings as _
    )
    .fetch_one(pool)
    .await
}

pub async fn update_user(
    username: &str,
    scoring_settings: &ScoringSettings,
) -> Result<Option<User>, sqlx::Error> {
    let pool = get_pool()?;
    sqlx::query_as!(
        User,
        r#"
        UPDATE users 
        SET scoring_settings = $1 
        WHERE username = $2
        RETURNING id, username, scoring_settings as "scoring_settings!: ScoringSettings", created_at
        "#,
        scoring_settings as _,
        username
    )
    .fetch_optional(pool)
    .await
}

pub async fn get_player(player_id: i32, user_id: i32) -> Result<PlayerDenormalized, Error> {
    let pool = get_pool()?;
    sqlx::query_as!(
        PlayerDenormalized,
        r#"
        SELECT 
            p.id,
            p.name,
            p.position as "position!: Position",
            p.team as "team!: Team",
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
            CASE u.scoring_settings
                WHEN 'Standard' THEN s.standard_pts
                WHEN 'Half' THEN s.half_ppr_pts
                WHEN 'PPR' THEN s.ppr_pts
            END as points,
            CASE u.scoring_settings
                WHEN 'Standard' THEN s.standard_pts_per_game
                WHEN 'Half' THEN s.half_ppr_pts_per_game
                WHEN 'PPR' THEN s.ppr_pts_per_game
            END as points_per_game
        FROM players p
        INNER JOIN users u ON u.id = $2
        LEFT JOIN rankings r ON p.id = r.player_id 
            AND r.scoring_settings = u.scoring_settings
        LEFT JOIN stats s ON p.id = s.player_id
        WHERE p.id = $1
        "#,
        player_id,
        user_id
    )
    .fetch_optional(pool)
    .await?
    .ok_or(sqlx::Error::RowNotFound)
}

pub async fn get_players(
    user_id: i32,
    position: Option<&Position>,
    team: Option<&Team>,
    name: Option<&str>,
    drafted: Option<bool>,
) -> Result<Vec<PlayerDenormalized>, Error> {
    let pool = get_pool()?;
    let players = sqlx::query_as!(
        PlayerDenormalized,
        r#"
        SELECT 
            p.id,
            p.name,
            p.position as "position!: Position",
            p.team as "team!: Team",
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
            CASE u.scoring_settings
                WHEN 'Standard' THEN s.standard_pts
                WHEN 'Half' THEN s.half_ppr_pts
                WHEN 'PPR' THEN s.ppr_pts
            END as points,
            CASE u.scoring_settings
                WHEN 'Standard' THEN s.standard_pts_per_game
                WHEN 'Half' THEN s.half_ppr_pts_per_game
                WHEN 'PPR' THEN s.ppr_pts_per_game
            END as points_per_game
        FROM players p
        INNER JOIN users u ON u.id = $1
        LEFT JOIN rankings r ON p.id = r.player_id 
            AND r.scoring_settings = u.scoring_settings
        LEFT JOIN stats s ON p.id = s.player_id
        LEFT JOIN drafted_players d ON d.user_id = $1
            AND p.id = d.player_id
        WHERE ($2::position_type IS NULL OR p.position = $2::position_type)
        AND ($3::team_type IS NULL OR p.team = $3::team_type)
        AND ($4::text IS NULL OR p.name ILIKE '%' || $4 || '%')
        AND (
            $5::boolean IS NULL 
            OR ($5 = true AND d.player_id IS NOT NULL)
            OR ($5 = false AND d.player_id IS NULL)
        )
        ORDER BY overall_ranking ASC
        "#,
        user_id,
        position as Option<&Position>,
        team as Option<&Team>,
        name,
        drafted
    )
    .fetch_all(pool)
    .await?;

    Ok(players)
}
