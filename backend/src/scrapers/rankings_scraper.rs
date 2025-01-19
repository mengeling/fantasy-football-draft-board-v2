use anyhow::Result;
use headless_chrome::Tab;
use regex::Regex;
use scraper::{Html, Selector};
use std::str::FromStr;

use crate::models::players::{PlayerIdentity, PlayerTask, Position, Team};
use crate::models::rankings::{Rankings, RankingsBase, ScoringSettings};

pub struct RankingsScraper<'a> {
    tab: &'a Tab,
}

impl<'a> RankingsScraper<'a> {
    pub fn new(tab: &'a Tab) -> Self {
        Self { tab }
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
        let mut ranking_tables = Vec::new();

        for (scoring_settings, url) in Self::get_urls() {
            self.tab.navigate_to(url)?;
            self.tab.wait_until_navigated()?;
            self.tab.wait_for_element("table#ranking-table")?;

            self.tab.evaluate(
                "let rows = document.querySelectorAll('tbody tr.player-row');
                    let lastRow = rows[rows.length - 1];
                    lastRow.scrollIntoView();",
                false,
            )?;

            let ranking_table = self.tab.wait_for_element("table#ranking-table")?;
            let ranking_table_html =
                ranking_table.call_js_fn("function() { return this.outerHTML; }", vec![], false)?;
            let ranking_table_html_value = ranking_table_html
                .value
                .and_then(|v| v.as_str().map(|s| s.to_string()))
                .unwrap();

            ranking_tables.push((ranking_table_html_value, scoring_settings));
        }
        self.tab.close(true)?;

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

            let overall_ranking = cells[0]
                .text()
                .collect::<String>()
                .parse::<i32>()
                .expect("Overall ranking should always be present");
            let player_identity = get_player_identity(&cells[2]);
            let (position, position_ranking) = get_position_ranking(&cells[3], &ranking_regex);
            // TODO: Click on cell 3 to open player bio modal and get bye week
            let bye_week = None;
            let best_ranking = cells[4]
                .text()
                .collect::<String>()
                .parse::<i32>()
                .expect("Best ranking should always be present");
            let worst_ranking = cells[5]
                .text()
                .collect::<String>()
                .parse::<i32>()
                .expect("Worst ranking should always be present");
            let average_ranking = cells[6]
                .text()
                .collect::<String>()
                .parse::<f32>()
                .expect("Average ranking should always be present");
            let standard_deviation_ranking = cells[7]
                .text()
                .collect::<String>()
                .parse::<f32>()
                .expect("Standard deviation ranking should always be present");

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
                    player_id: player_identity.id,
                    identity: player_identity,
                    position: position.clone(),
                    bye_week,
                });
            }
        }

        Ok((rankings, player_tasks))
    }
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
