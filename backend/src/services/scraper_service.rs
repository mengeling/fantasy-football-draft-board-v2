use anyhow::Result;
use headless_chrome::Browser;
use sqlx::QueryBuilder;

use crate::database::connection::DB_POOL;
use crate::models::player::Player;
use crate::models::rankings::Rankings;
use crate::models::stats::Stats;
use crate::scrapers::{
    player_scraper::PlayerScraper, rankings_scraper::RankingsScraper, stats_scraper::StatsScraper,
};

pub async fn run_scrapers() -> Result<()> {
    let sql = include_str!("../database/refresh_player_data.sql");
    sqlx::query(sql).execute(&*DB_POOL).await?;

    let browser = Browser::default()?;
    let tab = browser.new_tab()?;

    let rankings_scraper = RankingsScraper::new(&tab);
    let (rankings, player_tasks) = rankings_scraper.scrape().await?;

    let players = PlayerScraper::process_tasks(player_tasks).await?;

    let stats_scraper = StatsScraper::new();
    let stats = stats_scraper.scrape().await?;

    bulk_save_players(&players).await?;
    bulk_save_rankings(&rankings).await?;
    bulk_save_stats(&stats).await?;

    Ok(())
}

pub async fn bulk_save_players(players: &[Player]) -> Result<()> {
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

    query_builder.build().execute(&*DB_POOL).await?;
    Ok(())
}

pub async fn bulk_save_rankings(rankings: &[Rankings]) -> Result<()> {
    let mut query_builder =
        QueryBuilder::new("INSERT INTO rankings (player_id, scoring_settings, overall, position)");

    query_builder.push_values(rankings, |mut b, ranking| {
        b.push_bind(ranking.player_id)
            .push_bind(&ranking.scoring_settings)
            .push_bind(ranking.overall)
            .push_bind(ranking.position);
    });

    query_builder.build().execute(&*DB_POOL).await?;
    Ok(())
}

pub async fn bulk_save_stats(stats: &[Stats]) -> Result<()> {
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

    query_builder.build().execute(&*DB_POOL).await?;
    Ok(())
}
