use anyhow::Result;
use regex::Regex;
use reqwest::Client;
use scraper::{Html, Selector};
use url::Url;

use crate::constants::STATS_BY_POSITION;
use crate::models::stats::Stats;

pub struct StatsScraper {
    client: Client,
}

impl StatsScraper {
    pub fn new() -> Self {
        StatsScraper {
            client: Client::new(),
        }
    }

    fn build_url(&self, position: &str) -> Result<String> {
        let base_url = "https://www.fantasypros.com/nfl/stats/";
        let mut url = Url::parse(base_url)?;
        url.path_segments_mut()
            .map_err(|_| anyhow::anyhow!("Cannot modify URL"))?
            .push(position)
            .push("");

        Ok(url.to_string())
    }

    pub async fn scrape(&self) -> Result<Vec<Stats>> {
        let mut players: Vec<Stats> = Vec::new();

        for (position, headers) in STATS_BY_POSITION.iter() {
            let url = self.build_url(position)?;
            let response = self.client.get(url).send().await?;
            let html = Html::parse_document(&response.text().await?);

            let stats_table_selector = Selector::parse("table#data tbody").unwrap();
            let stats_row_selector = Selector::parse("tr").unwrap();
            let stats_cell_selector = Selector::parse("td").unwrap();

            if let Some(stats_table) = html.select(&stats_table_selector).next() {
                for row in stats_table.select(&stats_row_selector) {
                    let player_id = get_player_id(&row);

                    let mut current_stats = Stats {
                        player_id: player_id.unwrap(),
                        pass_cmp: 0.0,
                        pass_att: 0.0,
                        pass_cmp_pct: 0.0,
                        pass_yds: 0.0,
                        pass_yds_per_att: 0.0,
                        pass_td: 0.0,
                        pass_int: 0.0,
                        pass_sacks: 0.0,
                        rush_att: 0.0,
                        rush_yds: 0.0,
                        rush_yds_per_att: 0.0,
                        rush_long: 0.0,
                        rush_20: 0.0,
                        rush_td: 0.0,
                        fumbles: 0.0,
                        receptions: 0.0,
                        rec_tgt: 0.0,
                        rec_yds: 0.0,
                        rec_yds_per_rec: 0.0,
                        rec_long: 0.0,
                        rec_20: 0.0,
                        rec_td: 0.0,
                        field_goals: 0.0,
                        fg_att: 0.0,
                        fg_pct: 0.0,
                        fg_long: 0.0,
                        fg_1_19: 0.0,
                        fg_20_29: 0.0,
                        fg_30_39: 0.0,
                        fg_40_49: 0.0,
                        fg_50: 0.0,
                        extra_points: 0.0,
                        xp_att: 0.0,
                        sacks: 0.0,
                        int: 0.0,
                        fumbles_recovered: 0.0,
                        fumbles_forced: 0.0,
                        def_td: 0.0,
                        safeties: 0.0,
                        special_teams_td: 0.0,
                        games: 0.0,
                        standard_pts: 0.0,
                        standard_pts_per_game: 0.0,
                        half_ppr_pts: 0.0,
                        half_ppr_pts_per_game: 0.0,
                        ppr_pts: 0.0,
                        ppr_pts_per_game: 0.0,
                    };

                    for (cell_index, cell) in row.select(&stats_cell_selector).enumerate().skip(2) {
                        if cell_index < headers.len() + 2 {
                            let value = cell
                                .text()
                                .collect::<String>()
                                .parse::<f64>()
                                .unwrap_or(0.0);

                            match headers[cell_index - 2] {
                                "pass_cmp" => current_stats.pass_cmp = value,
                                "pass_att" => current_stats.pass_att = value,
                                "pass_cmp_pct" => current_stats.pass_cmp_pct = value,
                                "pass_yds" => current_stats.pass_yds = value,
                                "pass_yds_per_att" => current_stats.pass_yds_per_att = value,
                                "pass_td" => current_stats.pass_td = value,
                                "pass_int" => current_stats.pass_int = value,
                                "pass_sacks" => current_stats.pass_sacks = value,
                                "rush_att" => current_stats.rush_att = value,
                                "rush_yds" => current_stats.rush_yds = value,
                                "rush_yds_per_att" => current_stats.rush_yds_per_att = value,
                                "rush_long" => current_stats.rush_long = value,
                                "rush_20" => current_stats.rush_20 = value,
                                "rush_td" => current_stats.rush_td = value,
                                "fumbles" => current_stats.fumbles = value,
                                "receptions" => current_stats.receptions = value,
                                "rec_tgt" => current_stats.rec_tgt = value,
                                "rec_yds" => current_stats.rec_yds = value,
                                "rec_yds_per_rec" => current_stats.rec_yds_per_rec = value,
                                "rec_long" => current_stats.rec_long = value,
                                "rec_20" => current_stats.rec_20 = value,
                                "rec_td" => current_stats.rec_td = value,
                                "field_goals" => current_stats.field_goals = value,
                                "fg_att" => current_stats.fg_att = value,
                                "fg_pct" => current_stats.fg_pct = value,
                                "fg_long" => current_stats.fg_long = value,
                                "fg_1_19" => current_stats.fg_1_19 = value,
                                "fg_20_29" => current_stats.fg_20_29 = value,
                                "fg_30_39" => current_stats.fg_30_39 = value,
                                "fg_40_49" => current_stats.fg_40_49 = value,
                                "fg_50" => current_stats.fg_50 = value,
                                "extra_points" => current_stats.extra_points = value,
                                "xp_att" => current_stats.xp_att = value,
                                "sacks" => current_stats.sacks = value,
                                "int" => current_stats.int = value,
                                "fumbles_recovered" => current_stats.fumbles_recovered = value,
                                "fumbles_forced" => current_stats.fumbles_forced = value,
                                "def_td" => current_stats.def_td = value,
                                "safeties" => current_stats.safeties = value,
                                "special_teams_td" => current_stats.special_teams_td = value,
                                "games" => current_stats.games = value,
                                _ => (),
                            }
                        }
                    }

                    if current_stats.games > 0.0 {
                        current_stats.standard_pts = calculate_standard_points(&current_stats);
                        current_stats.standard_pts_per_game =
                            ((current_stats.standard_pts / current_stats.games) * 10.0).round()
                                / 10.0;

                        current_stats.half_ppr_pts = calculate_half_ppr_points(&current_stats);
                        current_stats.half_ppr_pts_per_game =
                            ((current_stats.half_ppr_pts / current_stats.games) * 10.0).round()
                                / 10.0;

                        current_stats.ppr_pts = calculate_ppr_points(&current_stats);
                        current_stats.ppr_pts_per_game =
                            ((current_stats.ppr_pts / current_stats.games) * 10.0).round() / 10.0;
                    }

                    if let Some(player_id) = player_id {
                        if let Some(existing_player) =
                            players.iter_mut().find(|p| p.player_id == player_id)
                        {
                            existing_player.pass_cmp =
                                existing_player.pass_cmp.max(current_stats.pass_cmp);
                            existing_player.pass_att =
                                existing_player.pass_att.max(current_stats.pass_att);
                            existing_player.pass_cmp_pct =
                                existing_player.pass_cmp_pct.max(current_stats.pass_cmp_pct);
                            existing_player.pass_yds =
                                existing_player.pass_yds.max(current_stats.pass_yds);
                            existing_player.pass_yds_per_att = existing_player
                                .pass_yds_per_att
                                .max(current_stats.pass_yds_per_att);
                            existing_player.pass_td =
                                existing_player.pass_td.max(current_stats.pass_td);
                            existing_player.pass_int =
                                existing_player.pass_int.max(current_stats.pass_int);
                            existing_player.pass_sacks =
                                existing_player.pass_sacks.max(current_stats.pass_sacks);
                            existing_player.rush_att =
                                existing_player.rush_att.max(current_stats.rush_att);
                            existing_player.rush_yds =
                                existing_player.rush_yds.max(current_stats.rush_yds);
                            existing_player.rush_yds_per_att = existing_player
                                .rush_yds_per_att
                                .max(current_stats.rush_yds_per_att);
                            existing_player.rush_long =
                                existing_player.rush_long.max(current_stats.rush_long);
                            existing_player.rush_20 =
                                existing_player.rush_20.max(current_stats.rush_20);
                            existing_player.rush_td =
                                existing_player.rush_td.max(current_stats.rush_td);
                            existing_player.fumbles =
                                existing_player.fumbles.max(current_stats.fumbles);
                            existing_player.receptions =
                                existing_player.receptions.max(current_stats.receptions);
                            existing_player.rec_tgt =
                                existing_player.rec_tgt.max(current_stats.rec_tgt);
                            existing_player.rec_yds =
                                existing_player.rec_yds.max(current_stats.rec_yds);
                            existing_player.rec_yds_per_rec = existing_player
                                .rec_yds_per_rec
                                .max(current_stats.rec_yds_per_rec);
                            existing_player.rec_long =
                                existing_player.rec_long.max(current_stats.rec_long);
                            existing_player.rec_20 =
                                existing_player.rec_20.max(current_stats.rec_20);
                            existing_player.rec_td =
                                existing_player.rec_td.max(current_stats.rec_td);
                            existing_player.field_goals =
                                existing_player.field_goals.max(current_stats.field_goals);
                            existing_player.fg_att =
                                existing_player.fg_att.max(current_stats.fg_att);
                            existing_player.fg_pct =
                                existing_player.fg_pct.max(current_stats.fg_pct);
                            existing_player.fg_long =
                                existing_player.fg_long.max(current_stats.fg_long);
                            existing_player.fg_1_19 =
                                existing_player.fg_1_19.max(current_stats.fg_1_19);
                            existing_player.fg_20_29 =
                                existing_player.fg_20_29.max(current_stats.fg_20_29);
                            existing_player.fg_30_39 =
                                existing_player.fg_30_39.max(current_stats.fg_30_39);
                            existing_player.fg_40_49 =
                                existing_player.fg_40_49.max(current_stats.fg_40_49);
                            existing_player.fg_50 = existing_player.fg_50.max(current_stats.fg_50);
                            existing_player.extra_points =
                                existing_player.extra_points.max(current_stats.extra_points);
                            existing_player.xp_att =
                                existing_player.xp_att.max(current_stats.xp_att);
                            existing_player.sacks = existing_player.sacks.max(current_stats.sacks);
                            existing_player.int = existing_player.int.max(current_stats.int);
                            existing_player.fumbles_recovered = existing_player
                                .fumbles_recovered
                                .max(current_stats.fumbles_recovered);
                            existing_player.fumbles_forced = existing_player
                                .fumbles_forced
                                .max(current_stats.fumbles_forced);
                            existing_player.def_td =
                                existing_player.def_td.max(current_stats.def_td);
                            existing_player.safeties =
                                existing_player.safeties.max(current_stats.safeties);
                            existing_player.special_teams_td = existing_player
                                .special_teams_td
                                .max(current_stats.special_teams_td);
                            existing_player.games = existing_player.games.max(current_stats.games);
                            existing_player.standard_pts =
                                existing_player.standard_pts.max(current_stats.standard_pts);
                            existing_player.standard_pts_per_game = existing_player
                                .standard_pts_per_game
                                .max(current_stats.standard_pts_per_game);
                            existing_player.half_ppr_pts =
                                existing_player.half_ppr_pts.max(current_stats.half_ppr_pts);
                            existing_player.half_ppr_pts_per_game = existing_player
                                .half_ppr_pts_per_game
                                .max(current_stats.half_ppr_pts_per_game);
                            existing_player.ppr_pts =
                                existing_player.ppr_pts.max(current_stats.ppr_pts);
                            existing_player.ppr_pts_per_game = existing_player
                                .ppr_pts_per_game
                                .max(current_stats.ppr_pts_per_game);
                        } else {
                            players.push(current_stats);
                        }
                    }
                }
            }
        }

        Ok(players)
    }
}

fn get_player_id(row: &scraper::element_ref::ElementRef) -> Option<i32> {
    let row_class = row.value().attr("class").unwrap_or("");
    Regex::new(r"(\d+)")
        .unwrap()
        .captures(row_class)
        .and_then(|cap| cap.get(1))
        .and_then(|m| m.as_str().parse::<i32>().ok())
}

fn calculate_standard_points(stats: &Stats) -> f64 {
    let mut points = 0.0;
    points += stats.pass_yds * 0.04;
    points += stats.pass_td * 4.0;
    points += stats.pass_int * -2.0;
    points += stats.rush_yds * 0.1;
    points += stats.rush_td * 6.0;
    points += stats.fumbles * -2.0;
    points += stats.rec_yds * 0.1;
    points += stats.rec_td * 6.0;
    points += stats.fg_1_19 * 3.0;
    points += stats.fg_20_29 * 3.0;
    points += stats.fg_30_39 * 3.0;
    points += stats.fg_40_49 * 4.0;
    points += stats.fg_50 * 5.0;
    points += stats.extra_points * 1.0;
    points += stats.sacks * 1.0;
    points += stats.int * 2.0;
    points += stats.fumbles_recovered * 2.0;
    points += stats.def_td * 6.0;
    points += stats.safeties * 2.0;
    points += stats.special_teams_td * 6.0;
    points
}

fn calculate_half_ppr_points(stats: &Stats) -> f64 {
    calculate_standard_points(stats) + (stats.receptions * 0.5)
}

fn calculate_ppr_points(stats: &Stats) -> f64 {
    calculate_standard_points(stats) + stats.receptions
}
