use anyhow::Result;
use sqlx::{Error, Postgres, QueryBuilder, Transaction};
use time::OffsetDateTime;

use crate::database::connection::get_pool;
use crate::models::drafted_players::DraftedPlayer;
use crate::models::players::{Player, PlayerResponse, Position, Team};
use crate::models::rankings::{Rankings, ScoringSettings};
use crate::models::stats::Stats;
use crate::models::users::User;

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
        if players.is_empty() {
            return Ok(());
        }

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
        if rankings.is_empty() {
            return Ok(());
        }

        let mut query_builder = QueryBuilder::new(
            "INSERT INTO rankings (
                player_id, scoring_settings, overall, position, 
                best, worst, average, standard_deviation
            )",
        );

        query_builder.push_values(rankings, |mut b, ranking| {
            b.push_bind(ranking.player_id)
                .push_bind(&ranking.scoring_settings)
                .push_bind(ranking.base.overall)
                .push_bind(ranking.base.position)
                .push_bind(ranking.base.best)
                .push_bind(ranking.base.worst)
                .push_bind(ranking.base.average)
                .push_bind(ranking.base.standard_deviation);
        });

        query_builder.build().execute(&mut **tx).await?;
        Ok(())
    }

    pub async fn bulk_save_stats(
        stats: &[Stats],
        tx: &mut Transaction<'_, Postgres>,
    ) -> Result<()> {
        if stats.is_empty() {
            return Ok(());
        }

        let mut query_builder = QueryBuilder::new(
            "INSERT INTO stats (
                player_id, pass_cmp, pass_att, pass_cmp_pct, pass_yds, pass_yds_per_att,
                pass_td, pass_int, pass_sacks, rush_att, rush_yds, rush_yds_per_att,
                rush_long, rush_20, rush_td, fumbles, receptions, rec_tgt, rec_tgt_pct, rec_yds,
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
                .push_bind(stat.base.pass_cmp)
                .push_bind(stat.base.pass_att)
                .push_bind(stat.base.pass_cmp_pct)
                .push_bind(stat.base.pass_yds)
                .push_bind(stat.base.pass_yds_per_att)
                .push_bind(stat.base.pass_td)
                .push_bind(stat.base.pass_int)
                .push_bind(stat.base.pass_sacks)
                .push_bind(stat.base.rush_att)
                .push_bind(stat.base.rush_yds)
                .push_bind(stat.base.rush_yds_per_att)
                .push_bind(stat.base.rush_long)
                .push_bind(stat.base.rush_20)
                .push_bind(stat.base.rush_td)
                .push_bind(stat.base.fumbles)
                .push_bind(stat.base.receptions)
                .push_bind(stat.base.rec_tgt)
                .push_bind(stat.base.rec_tgt_pct)
                .push_bind(stat.base.rec_yds)
                .push_bind(stat.base.rec_yds_per_rec)
                .push_bind(stat.base.rec_long)
                .push_bind(stat.base.rec_20)
                .push_bind(stat.base.rec_td)
                .push_bind(stat.base.field_goals)
                .push_bind(stat.base.fg_att)
                .push_bind(stat.base.fg_pct)
                .push_bind(stat.base.fg_long)
                .push_bind(stat.base.fg_1_19)
                .push_bind(stat.base.fg_20_29)
                .push_bind(stat.base.fg_30_39)
                .push_bind(stat.base.fg_40_49)
                .push_bind(stat.base.fg_50)
                .push_bind(stat.base.extra_points)
                .push_bind(stat.base.xp_att)
                .push_bind(stat.base.sacks)
                .push_bind(stat.base.int)
                .push_bind(stat.base.fumbles_recovered)
                .push_bind(stat.base.fumbles_forced)
                .push_bind(stat.base.def_td)
                .push_bind(stat.base.safeties)
                .push_bind(stat.base.special_teams_td)
                .push_bind(stat.base.games)
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

    pub async fn get_players(user_id: i32) -> Result<Vec<PlayerResponse>, Error> {
        let pool = get_pool()?;
        sqlx::query_as!(
            PlayerResponse,
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
                jsonb_build_object(
                    'overall', r.overall,
                    'position', r.position,
                    'best', r.best,
                    'worst', r.worst,
                    'average', r.average,
                    'standard_deviation', r.standard_deviation
                ) as "rankings!: serde_json::Value",
                jsonb_build_object(
                    'pass_cmp', COALESCE(s.pass_cmp, 0),
                    'pass_att', COALESCE(s.pass_att, 0),
                    'pass_cmp_pct', COALESCE(s.pass_cmp_pct, 0),
                    'pass_yds', COALESCE(s.pass_yds, 0),
                    'pass_yds_per_att', COALESCE(s.pass_yds_per_att, 0),
                    'pass_td', COALESCE(s.pass_td, 0),
                    'pass_int', COALESCE(s.pass_int, 0),
                    'pass_sacks', COALESCE(s.pass_sacks, 0),
                    'rush_att', COALESCE(s.rush_att, 0),
                    'rush_yds', COALESCE(s.rush_yds, 0),
                    'rush_yds_per_att', COALESCE(s.rush_yds_per_att, 0),
                    'rush_long', COALESCE(s.rush_long, 0),
                    'rush_20', COALESCE(s.rush_20, 0),
                    'rush_td', COALESCE(s.rush_td, 0),
                    'fumbles', COALESCE(s.fumbles, 0),
                    'receptions', COALESCE(s.receptions, 0),
                    'rec_tgt', COALESCE(s.rec_tgt, 0),
                    'rec_tgt_pct', COALESCE(s.rec_tgt_pct, 0),
                    'rec_yds', COALESCE(s.rec_yds, 0),
                    'rec_yds_per_rec', COALESCE(s.rec_yds_per_rec, 0),
                    'rec_long', COALESCE(s.rec_long, 0),
                    'rec_20', COALESCE(s.rec_20, 0),
                    'rec_td', COALESCE(s.rec_td, 0),
                    'field_goals', COALESCE(s.field_goals, 0),
                    'fg_att', COALESCE(s.fg_att, 0),
                    'fg_pct', COALESCE(s.fg_pct, 0),
                    'fg_long', COALESCE(s.fg_long, 0),
                    'fg_1_19', COALESCE(s.fg_1_19, 0),
                    'fg_20_29', COALESCE(s.fg_20_29, 0),
                    'fg_30_39', COALESCE(s.fg_30_39, 0),
                    'fg_40_49', COALESCE(s.fg_40_49, 0),
                    'fg_50', COALESCE(s.fg_50, 0),
                    'extra_points', COALESCE(s.extra_points, 0),
                    'xp_att', COALESCE(s.xp_att, 0),
                    'sacks', COALESCE(s.sacks, 0),
                    'int', COALESCE(s.int, 0),
                    'fumbles_recovered', COALESCE(s.fumbles_recovered, 0),
                    'fumbles_forced', COALESCE(s.fumbles_forced, 0),
                    'def_td', COALESCE(s.def_td, 0),
                    'safeties', COALESCE(s.safeties, 0),
                    'special_teams_td', COALESCE(s.special_teams_td, 0),
                    'games', COALESCE(s.games, 0),
                    'points', CASE u.scoring_settings
                        WHEN 'Standard' THEN s.standard_pts
                        WHEN 'Half' THEN s.half_ppr_pts
                        WHEN 'PPR' THEN s.ppr_pts
                    END,
                    'points_per_game', CASE u.scoring_settings
                        WHEN 'Standard' THEN s.standard_pts_per_game
                        WHEN 'Half' THEN s.half_ppr_pts_per_game
                        WHEN 'PPR' THEN s.ppr_pts_per_game
                    END
                ) as "stats!: serde_json::Value",
                d.player_id IS NOT NULL as "drafted!: bool"
            FROM players p
            INNER JOIN users u ON u.id = $1
            INNER JOIN rankings r ON p.id = r.player_id 
                AND r.scoring_settings = u.scoring_settings
            LEFT JOIN stats s ON p.id = s.player_id
            LEFT JOIN drafted_players d ON d.user_id = $1
                AND p.id = d.player_id
            ORDER BY r.overall ASC
            "#,
            user_id,
        )
        .fetch_all(pool)
        .await
    }
}

pub mod drafted_player_operations {
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

    pub async fn undraft_all(user_id: i32) -> Result<u64, Error> {
        let pool = get_pool()?;
        let result = sqlx::query!(
            r#"
            DELETE FROM drafted_players
            WHERE user_id = $1
            "#,
            user_id
        )
        .execute(pool)
        .await?;

        Ok(result.rows_affected())
    }
}
