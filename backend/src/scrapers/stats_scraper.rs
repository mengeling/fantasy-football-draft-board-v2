use anyhow::Result;
use headless_chrome::Tab;
use regex::Regex;
use round::round;
use scraper::{Html, Selector};
use std::time::{Duration, Instant};

use crate::constants::STATS_BY_POSITION;
use crate::models::stats::Stats;

pub struct StatsScraper<'a> {
    tab: &'a Tab,
}

impl<'a> StatsScraper<'a> {
    pub fn new(tab: &'a Tab) -> Self {
        StatsScraper { tab }
    }

    fn build_url(&self, position: &str) -> String {
        format!("https://www.fantasypros.com/nfl/stats/{}.php", position)
    }

    pub async fn scrape(&self) -> Result<Vec<Stats>> {
        let mut players: Vec<Stats> = Vec::new();

        for (position, headers) in STATS_BY_POSITION.iter() {
            let url = self.build_url(position);
            let table_html = self.load_stats_table(&url)?;
            let html = Html::parse_document(&table_html);

            let stats_table_selector = Selector::parse("table#data tbody").unwrap();
            let stats_row_selector = Selector::parse("tr").unwrap();
            let stats_cell_selector = Selector::parse("td").unwrap();

            if let Some(stats_table) = html.select(&stats_table_selector).next() {
                for row in stats_table.select(&stats_row_selector) {
                    let player_id = match get_player_id(&row) {
                        Some(id) => id,
                        None => continue,
                    };
                    let mut current_stats = Stats::new(player_id);

                    for (cell_index, cell) in row.select(&stats_cell_selector).enumerate().skip(2) {
                        if cell_index < headers.len() + 2 {
                            let value = cell
                                .text()
                                .collect::<String>()
                                .replace(',', "")
                                .trim()
                                .parse::<f64>()
                                .unwrap_or(0.0);

                            match headers[cell_index - 2] {
                                "pass_cmp" => current_stats.base.pass_cmp = value,
                                "pass_att" => current_stats.base.pass_att = value,
                                "pass_cmp_pct" => current_stats.base.pass_cmp_pct = value,
                                "pass_yds" => current_stats.base.pass_yds = value,
                                "pass_yds_per_att" => current_stats.base.pass_yds_per_att = value,
                                "pass_td" => current_stats.base.pass_td = value,
                                "pass_int" => current_stats.base.pass_int = value,
                                "pass_sacks" => current_stats.base.pass_sacks = value,
                                "rush_att" => current_stats.base.rush_att = value,
                                "rush_yds" => current_stats.base.rush_yds = value,
                                "rush_yds_per_att" => current_stats.base.rush_yds_per_att = value,
                                "rush_long" => current_stats.base.rush_long = value,
                                "rush_20" => current_stats.base.rush_20 = value,
                                "rush_td" => current_stats.base.rush_td = value,
                                "fumbles" => current_stats.base.fumbles = value,
                                "receptions" => current_stats.base.receptions = value,
                                "rec_tgt" => current_stats.base.rec_tgt = value,
                                "rec_yds" => current_stats.base.rec_yds = value,
                                "rec_yds_per_rec" => current_stats.base.rec_yds_per_rec = value,
                                "rec_long" => current_stats.base.rec_long = value,
                                "rec_20" => current_stats.base.rec_20 = value,
                                "rec_td" => current_stats.base.rec_td = value,
                                "field_goals" => current_stats.base.field_goals = value,
                                "fg_att" => current_stats.base.fg_att = value,
                                "fg_pct" => current_stats.base.fg_pct = value,
                                "fg_long" => current_stats.base.fg_long = value,
                                "fg_1_19" => current_stats.base.fg_1_19 = value,
                                "fg_20_29" => current_stats.base.fg_20_29 = value,
                                "fg_30_39" => current_stats.base.fg_30_39 = value,
                                "fg_40_49" => current_stats.base.fg_40_49 = value,
                                "fg_50" => current_stats.base.fg_50 = value,
                                "extra_points" => current_stats.base.extra_points = value,
                                "xp_att" => current_stats.base.xp_att = value,
                                "sacks" => current_stats.base.sacks = value,
                                "int" => current_stats.base.int = value,
                                "fumbles_recovered" => current_stats.base.fumbles_recovered = value,
                                "fumbles_forced" => current_stats.base.fumbles_forced = value,
                                "def_td" => current_stats.base.def_td = value,
                                "safeties" => current_stats.base.safeties = value,
                                "special_teams_td" => current_stats.base.special_teams_td = value,
                                "games" => current_stats.base.games = value,
                                _ => (),
                            }
                        }
                    }

                    if current_stats.base.games > 0.0 {
                        current_stats.standard_pts = calculate_standard_points(&current_stats);
                        current_stats.standard_pts_per_game =
                            round(current_stats.standard_pts / current_stats.base.games, 1);

                        current_stats.half_ppr_pts = calculate_half_ppr_points(&current_stats);
                        current_stats.half_ppr_pts_per_game =
                            round(current_stats.half_ppr_pts / current_stats.base.games, 1);

                        current_stats.ppr_pts = calculate_ppr_points(&current_stats);
                        current_stats.ppr_pts_per_game =
                            round(current_stats.ppr_pts / current_stats.base.games, 1);
                    }

                    if let Some(existing_player) =
                        players.iter_mut().find(|p| p.player_id == player_id)
                    {
                        existing_player.update_from(&current_stats);
                    } else {
                        players.push(current_stats);
                    }
                }
            }
        }

        Ok(players)
    }

    fn load_stats_table(&self, url: &str) -> Result<String> {
        const MAX_ATTEMPTS: u32 = 3;
        let mut last_err: Option<anyhow::Error> = None;

        for attempt in 1..=MAX_ATTEMPTS {
            match self.try_load_stats_table(url) {
                Ok(html) => return Ok(html),
                Err(e) => {
                    eprintln!(
                        "Stats scrape attempt {}/{} for {} failed: {}",
                        attempt, MAX_ATTEMPTS, url, e
                    );
                    last_err = Some(e);
                    std::thread::sleep(Duration::from_secs(2));
                }
            }
        }

        Err(last_err.unwrap_or_else(|| anyhow::anyhow!("Failed to scrape {}", url)))
    }

    fn try_load_stats_table(&self, url: &str) -> Result<String> {
        self.tab.navigate_to(url)?;
        self.tab.wait_until_navigated()?;
        self.dismiss_consent_banner();
        self.wait_until_stats_rendered(Duration::from_secs(20))?;

        let table = self.tab.wait_for_element("table#data")?;
        let html = table.get_content()?;

        let row_count = Html::parse_document(&html)
            .select(&Selector::parse("table#data tbody tr").unwrap())
            .count();
        if row_count == 0 {
            return Err(anyhow::anyhow!("Stats table captured with 0 rows"));
        }

        Ok(html)
    }

    fn dismiss_consent_banner(&self) {
        // Best-effort: dismiss the OneTrust cookie banner shown on fresh
        // sessions. Prefer rejecting non-essential cookies.
        let _ = self.tab.evaluate(
            r#"(function () {
                var b = document.querySelector(
                    '#onetrust-reject-all-handler, #onetrust-accept-btn-handler, .onetrust-close-btn-handler'
                );
                if (b) { b.click(); return true; }
                return false;
            })()"#,
            false,
        );
    }

    fn wait_until_stats_rendered(&self, timeout: Duration) -> Result<()> {
        let check = r#"(function () {
            return document.querySelectorAll('table#data tbody tr').length > 0;
        })()"#;

        let start = Instant::now();
        loop {
            let ready = self
                .tab
                .evaluate(check, false)?
                .value
                .and_then(|v| v.as_bool())
                .unwrap_or(false);
            if ready {
                return Ok(());
            }
            if start.elapsed() >= timeout {
                return Err(anyhow::anyhow!(
                    "Stats table did not render within {:?}",
                    timeout
                ));
            }
            std::thread::sleep(Duration::from_millis(400));
        }
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
    0.0 + stats.base.pass_yds * 0.04
        + stats.base.pass_td * 4.0
        + stats.base.pass_int * -2.0
        + stats.base.rush_yds * 0.1
        + stats.base.rush_td * 6.0
        + stats.base.fumbles * -2.0
        + stats.base.rec_yds * 0.1
        + stats.base.rec_td * 6.0
        + stats.base.fg_1_19 * 3.0
        + stats.base.fg_20_29 * 3.0
        + stats.base.fg_30_39 * 3.0
        + stats.base.fg_40_49 * 4.0
        + stats.base.fg_50 * 5.0
        + stats.base.extra_points * 1.0
        + stats.base.sacks * 1.0
        + stats.base.int * 2.0
        + stats.base.fumbles_recovered * 2.0
        + stats.base.def_td * 6.0
        + stats.base.safeties * 2.0
        + stats.base.special_teams_td * 6.0
}

fn calculate_half_ppr_points(stats: &Stats) -> f64 {
    stats.standard_pts + (stats.base.receptions * 0.5)
}

fn calculate_ppr_points(stats: &Stats) -> f64 {
    stats.standard_pts + stats.base.receptions
}
