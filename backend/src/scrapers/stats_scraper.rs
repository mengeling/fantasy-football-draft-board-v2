use anyhow::Result;
use regex::Regex;
use reqwest::header::{HeaderMap, HeaderValue, ACCEPT, ACCEPT_LANGUAGE};
use reqwest::Client;
use round::round;
use scraper::{Html, Selector};
use std::time::Duration;

use crate::constants::STATS_BY_POSITION;
use crate::models::stats::Stats;

const BROWSER_USER_AGENT: &str = "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) \
AppleWebKit/537.36 (KHTML, like Gecko) Chrome/126.0.0.0 Safari/537.36";

pub struct StatsScraper {
    client: Client,
}

impl StatsScraper {
    pub fn new() -> Self {
        // FantasyPros' CloudFront edge returns 403 to requests that don't look
        // like a browser (notably the default `Accept: */*`), so send a
        // browser-like Accept + User-Agent. The stats tables are server-rendered,
        // so a plain HTTP GET returns the full table with no JS required.
        let mut default_headers = HeaderMap::new();
        default_headers.insert(
            ACCEPT,
            HeaderValue::from_static(
                "text/html,application/xhtml+xml,application/xml;q=0.9,*/*;q=0.8",
            ),
        );
        default_headers.insert(ACCEPT_LANGUAGE, HeaderValue::from_static("en-US,en;q=0.9"));

        let client = Client::builder()
            .user_agent(BROWSER_USER_AGENT)
            .default_headers(default_headers)
            .cookie_store(true)
            .build()
            .expect("Failed to build stats scraper HTTP client");

        StatsScraper { client }
    }

    fn build_url(&self, position: &str) -> String {
        format!("https://www.fantasypros.com/nfl/stats/{}.php", position)
    }

    pub async fn scrape(&self) -> Result<Vec<Stats>> {
        let mut players: Vec<Stats> = Vec::new();

        for (index, (position, headers)) in STATS_BY_POSITION.iter().enumerate() {
            // Space requests out slightly so the burst doesn't look like a bot.
            if index > 0 {
                tokio::time::sleep(Duration::from_millis(500)).await;
            }

            let url = self.build_url(position);
            let body = self.fetch_stats_page(&url).await?;
            let html = Html::parse_document(&body);

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

    async fn fetch_stats_page(&self, url: &str) -> Result<String> {
        const MAX_ATTEMPTS: u32 = 4;
        let mut last_err: Option<anyhow::Error> = None;

        for attempt in 1..=MAX_ATTEMPTS {
            match self.client.get(url).send().await {
                Ok(response) => {
                    let status = response.status();
                    let body = response.text().await?;
                    if status.is_success() {
                        return Ok(body);
                    }
                    eprintln!(
                        "Stats request attempt {}/{} for {} returned HTTP {}",
                        attempt, MAX_ATTEMPTS, url, status
                    );
                    last_err = Some(anyhow::anyhow!(
                        "Stats request for {} returned HTTP {}",
                        url,
                        status
                    ));
                }
                Err(e) => {
                    eprintln!(
                        "Stats request attempt {}/{} for {} failed: {}",
                        attempt, MAX_ATTEMPTS, url, e
                    );
                    last_err = Some(e.into());
                }
            }

            tokio::time::sleep(Duration::from_millis(1000 * attempt as u64)).await;
        }

        Err(last_err.unwrap_or_else(|| anyhow::anyhow!("Failed to fetch {}", url)))
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
