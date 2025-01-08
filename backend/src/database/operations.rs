use anyhow::Result;
use sqlx::{Error, Postgres, QueryBuilder, Transaction};
use time::OffsetDateTime;

use crate::database::connection::get_pool;
use crate::models::drafted_player::DraftedPlayer;
use crate::models::player::{Player, PlayerDenormalized, Position, Team};
use crate::models::rankings::{Rankings, ScoringSettings};
use crate::models::stats::Stats;
use crate::models::user::User;

pub mod fantasy_data_operations {
    use super::*;

    pub async fn delete_old_data(tx: &mut Transaction<'_, Postgres>) -> Result<()> {
        sqlx::query!("DELETE FROM players")
            .execute(&mut **tx)
            .await?;
        sqlx::query!("DELETE FROM rankings")
            .execute(&mut **tx)
            .await?;
        sqlx::query!("DELETE FROM stats").execute(&mut **tx).await?;
        Ok(())
    }

    pub async fn record_fantasy_data_update(tx: &mut Transaction<'_, Postgres>) -> Result<()> {
        sqlx::query("INSERT INTO fantasy_data_updates DEFAULT VALUES")
            .execute(&mut **tx)
            .await?;
        Ok(())
    }

    pub async fn get_last_fantasy_data_update() -> Result<Option<OffsetDateTime>, Error> {
        let pool = get_pool()?;
        sqlx::query_scalar!(
            r#"
            SELECT completed_at
            FROM fantasy_data_updates
            ORDER BY completed_at DESC
            LIMIT 1
            "#
        )
        .fetch_optional(pool)
        .await
    }

    pub async fn bulk_save_players(
        players: &[Player],
        tx: &mut Transaction<'_, Postgres>,
    ) -> Result<()> {
        let mut query_builder = QueryBuilder::new(
            "INSERT INTO players (id, name, position, team, bye_week, height, weight, age, college)",
        );

        query_builder.push_values(players, |mut b, player| {
            b.push_bind(player.id)
                .push_bind(&player.name)
                .push_bind(&player.position)
                .push_bind(&player.team)
                .push_bind(player.bye_week)
                .push_bind(&player.height)
                .push_bind(&player.weight)
                .push_bind(player.age)
                .push_bind(&player.college);
        });

        query_builder.build().execute(&mut **tx).await?;
        Ok(())
    }

    pub async fn bulk_save_rankings(
        rankings: &[Rankings],
        tx: &mut Transaction<'_, Postgres>,
    ) -> Result<()> {
        let mut query_builder = QueryBuilder::new(
            "INSERT INTO rankings (player_id, scoring_settings, overall, position)",
        );

        query_builder.push_values(rankings, |mut b, ranking| {
            b.push_bind(ranking.player_id)
                .push_bind(&ranking.scoring_settings)
                .push_bind(ranking.overall)
                .push_bind(ranking.position);
        });

        query_builder.build().execute(&mut **tx).await?;
        Ok(())
    }

    pub async fn bulk_save_stats(
        stats: &[Stats],
        tx: &mut Transaction<'_, Postgres>,
    ) -> Result<()> {
        let mut query_builder = QueryBuilder::new(
            "INSERT INTO stats (
                player_id, pass_cmp, pass_att, pass_cmp_pct, pass_yds, pass_yds_per_att,
                pass_td, pass_int, pass_sacks, rush_att, rush_yds, rush_yds_per_att,
                rush_long, rush_20, rush_td, fumbles, receptions, rec_tgt, rec_yds,
                rec_yds_per_rec, rec_long, rec_20, rec_td, field_goals, fg_att,
                fg_pct, fg_long, fg_1_19, fg_20_29, fg_30_39, fg_40_49, fg_50,
                extra_points, xp_att, sacks, int, fumbles_recovered, fumbles_forced,
                def_td, safeties, special_teams_td, games, standard_pts, 
                standard_pts_per_game, half_ppr_pts, half_ppr_pts_per_game, 
                ppr_pts, ppr_pts_per_game
            )",
        );

        query_builder.push_values(stats, |mut b, stat| {
            b.push_bind(stat.player_id)
                .push_bind(stat.pass_cmp)
                .push_bind(stat.pass_att)
                .push_bind(stat.pass_cmp_pct)
                .push_bind(stat.pass_yds)
                .push_bind(stat.pass_yds_per_att)
                .push_bind(stat.pass_td)
                .push_bind(stat.pass_int)
                .push_bind(stat.pass_sacks)
                .push_bind(stat.rush_att)
                .push_bind(stat.rush_yds)
                .push_bind(stat.rush_yds_per_att)
                .push_bind(stat.rush_long)
                .push_bind(stat.rush_20)
                .push_bind(stat.rush_td)
                .push_bind(stat.fumbles)
                .push_bind(stat.receptions)
                .push_bind(stat.rec_tgt)
                .push_bind(stat.rec_yds)
                .push_bind(stat.rec_yds_per_rec)
                .push_bind(stat.rec_long)
                .push_bind(stat.rec_20)
                .push_bind(stat.rec_td)
                .push_bind(stat.field_goals)
                .push_bind(stat.fg_att)
                .push_bind(stat.fg_pct)
                .push_bind(stat.fg_long)
                .push_bind(stat.fg_1_19)
                .push_bind(stat.fg_20_29)
                .push_bind(stat.fg_30_39)
                .push_bind(stat.fg_40_49)
                .push_bind(stat.fg_50)
                .push_bind(stat.extra_points)
                .push_bind(stat.xp_att)
                .push_bind(stat.sacks)
                .push_bind(stat.int)
                .push_bind(stat.fumbles_recovered)
                .push_bind(stat.fumbles_forced)
                .push_bind(stat.def_td)
                .push_bind(stat.safeties)
                .push_bind(stat.special_teams_td)
                .push_bind(stat.games)
                .push_bind(stat.standard_pts)
                .push_bind(stat.standard_pts_per_game)
                .push_bind(stat.half_ppr_pts)
                .push_bind(stat.half_ppr_pts_per_game)
                .push_bind(stat.ppr_pts)
                .push_bind(stat.ppr_pts_per_game);
        });

        query_builder.build().execute(&mut **tx).await?;
        Ok(())
    }
}

pub mod user_operations {
    use super::*;

    pub async fn get_user(username: &str) -> Result<Option<User>, Error> {
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
    ) -> Result<User, Error> {
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
    ) -> Result<Option<User>, Error> {
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
}

pub mod player_operations {
    use super::*;

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
        .await
    }
}

pub mod draft_operations {
    use super::*;

    pub async fn draft_player(user_id: i32, player_id: i32) -> Result<DraftedPlayer, Error> {
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

    pub async fn undraft_player(user_id: i32, player_id: i32) -> Result<bool, Error> {
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
}
