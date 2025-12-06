use anyhow::Result;
use regex::Regex;
use scraper::{Html, Selector};
use std::str::FromStr;
use thirtyfour::prelude::*;

use crate::models::players::{PlayerIdentity, PlayerTask, Position, Team};
use crate::models::rankings::{Rankings, RankingsBase, ScoringSettings};

pub struct RankingsScraper<'a> {
    driver: &'a WebDriver,
}

impl<'a> RankingsScraper<'a> {
    pub fn new(driver: &'a WebDriver) -> Self {
        Self { driver }
    }

    fn get_urls() -> std::collections::HashMap<ScoringSettings, &'static str> {
        std::collections::HashMap::from([
            (
                ScoringSettings::Standard,
                "https://www.fantasypros.com/nfl/rankings/consensus-cheatsheets.php",
            ),
            (
                ScoringSettings::Half,
                "https://www.fantasypros.com/nfl/rankings/half-point-ppr-cheatsheets.php",
            ),
            (
                ScoringSettings::PPR,
                "https://www.fantasypros.com/nfl/rankings/ppr-cheatsheets.php",
            ),
        ])
    }

    pub async fn scrape(&self) -> Result<(Vec<Rankings>, Vec<PlayerTask>)> {
        let mut ranking_tables: Vec<(String, ScoringSettings)> = Vec::new();

        for (scoring_settings, url) in Self::get_urls() {
            self.driver.goto(url).await?;

            if let Ok(cookie_button) = self
                .driver
                .query(By::Css("#onetrust-accept-btn-handler"))
                .first()
                .await
            {
                let _ = cookie_button.click().await;
                tokio::time::sleep(tokio::time::Duration::from_millis(200)).await;
            }

            let view_dropdown_button = self
                .driver
                .query(By::Css(".select-advanced--view .select-advanced__button"))
                .first()
                .await?;
            view_dropdown_button.scroll_into_view().await?;
            view_dropdown_button.click().await?;

            self.driver
                .query(By::Css(
                    ".select-advanced--view .select-advanced__button.select-advanced__button--open",
                ))
                .first()
                .await?;

            let view_option_buttons = self
                .driver
                .find_all(By::Css(
                    ".select-advanced--view .select-advanced__item .select-advanced-content--button",
                ))
                .await?;
            let mut found_ranks_option = false;
            for option_button in &view_option_buttons {
                if option_button.text().await?.trim() == "Ranks" {
                    found_ranks_option = true;
                    option_button.click().await?;
                    tokio::time::sleep(tokio::time::Duration::from_millis(200)).await;
                    break;
                }
            }
            if !found_ranks_option {
                return Err(anyhow::anyhow!("Could not find 'Ranks' option in dropdown"));
            }

            self.driver
                .query(By::Css("table#ranking-table"))
                .first()
                .await?;
            let ranking_table_last_row = self
                .driver
                .query(By::Css("tbody tr.player-row:last-child"))
                .first()
                .await?;
            ranking_table_last_row.scroll_into_view().await?;

            let ranking_table = self
                .driver
                .query(By::Css("table#ranking-table"))
                .first()
                .await?;
            ranking_tables.push((
                ranking_table.outer_html().await?.to_string(),
                scoring_settings,
            ));
        }

        let mut seen_players = std::collections::HashSet::new();
        let mut all_rankings = Vec::new();
        let mut all_player_tasks = Vec::new();

        for (ranking_table, scoring_settings) in ranking_tables {
            let (rankings, player_tasks) = self
                .parse_ranking_table(&ranking_table, &mut seen_players, scoring_settings)
                .await?;
            all_rankings.extend(rankings);
            all_player_tasks.extend(player_tasks);
        }

        Ok((all_rankings, all_player_tasks))
    }

    async fn parse_ranking_table(
        &self,
        table_html: &str,
        seen_players: &mut std::collections::HashSet<i32>,
        scoring_settings: ScoringSettings,
    ) -> Result<(Vec<Rankings>, Vec<PlayerTask>)> {
        let document = Html::parse_document(table_html);
        let row_selector = Selector::parse("tbody tr.player-row").unwrap();
        let cell_selector = Selector::parse("td").unwrap();
        let ranking_regex = Regex::new(r"(\D+)(\d+)").unwrap();

        let mut rankings = Vec::new();
        let mut player_tasks = Vec::new();

        for row in document.select(&row_selector) {
            let cells: Vec<_> = row.select(&cell_selector).collect();
            let overall_ranking = parse_cell_as_number::<i32>(&cells[0], "Overall ranking");
            let player_identity = get_player_identity(&cells[2]);
            let (position, position_ranking) = get_position_ranking(&cells[3], &ranking_regex);
            let best_ranking = parse_cell_as_number::<i32>(&cells[4], "Best ranking");
            let worst_ranking = parse_cell_as_number::<i32>(&cells[5], "Worst ranking");
            let average_ranking = parse_cell_as_number::<f32>(&cells[6], "Average ranking");
            let standard_deviation_ranking =
                parse_cell_as_number::<f32>(&cells[7], "Standard deviation ranking");

            rankings.push(Rankings {
                player_id: player_identity.id,
                scoring_settings: scoring_settings.clone(),
                base: RankingsBase {
                    overall: overall_ranking,
                    position: position_ranking,
                    best: best_ranking,
                    worst: worst_ranking,
                    average: average_ranking,
                    standard_deviation: standard_deviation_ranking,
                },
            });

            if !seen_players.contains(&player_identity.id) {
                seen_players.insert(player_identity.id);
                player_tasks.push(PlayerTask {
                    identity: player_identity,
                    position: position.clone(),
                });
            }
        }

        Ok((rankings, player_tasks))
    }
}

fn parse_cell_as_number<T: FromStr>(
    cell: &scraper::element_ref::ElementRef,
    field_name: &str,
) -> T {
    cell.text()
        .collect::<String>()
        .parse()
        .unwrap_or_else(|_| panic!("{} should always be present", field_name))
}

fn get_player_identity(player_cell: &scraper::element_ref::ElementRef) -> PlayerIdentity {
    let player_id = player_cell
        .select(&Selector::parse("div").unwrap())
        .next()
        .unwrap()
        .value()
        .attr("data-player")
        .and_then(|s| s.parse::<i32>().ok())
        .expect("Player ID should always be present");
    let team = Team::from_str(
        player_cell
            .select(&Selector::parse("span").unwrap())
            .next()
            .unwrap()
            .text()
            .collect::<String>()
            .trim_matches(&['(', ')'][..]),
    )
    .unwrap();
    let player_url_element = player_cell
        .select(&Selector::parse("a").unwrap())
        .next()
        .unwrap();
    let bio_url = player_url_element
        .value()
        .attr("href")
        .unwrap_or("")
        .replace("/players/", "/schedule/")
        .to_string();
    let name = player_url_element.text().collect::<String>();

    PlayerIdentity {
        id: player_id,
        bio_url,
        name,
        team,
    }
}

fn get_position_ranking(
    position_cell: &scraper::element_ref::ElementRef,
    re: &Regex,
) -> (Position, i32) {
    let position_text = position_cell.text().collect::<String>();
    let caps = re
        .captures(&position_text)
        .expect("Position and ranking should always be present");
    (
        Position::from_str(&caps[1]).unwrap(),
        caps[2]
            .parse::<i32>()
            .expect("Position ranking should always be present"),
    )
}
