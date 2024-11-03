use anyhow::Result;
use headless_chrome::Tab;
use regex::Regex;
use scraper::{Html, Selector};

use crate::models::player::{Player, PlayerBio, PlayerIdentity};
use crate::models::rankings::Rankings;
use crate::scrapers::player_scraper::PlayerScraper;

pub struct RankingsScraper<'a> {
    tab: &'a Tab,
}

impl<'a> RankingsScraper<'a> {
    pub fn new(tab: &'a Tab) -> Self {
        Self { tab }
    }

    fn get_urls() -> std::collections::HashMap<&'static str, &'static str> {
        std::collections::HashMap::from([
            (
                "standard",
                "https://www.fantasypros.com/nfl/rankings/consensus-cheatsheets.php",
            ),
            (
                "half",
                "https://www.fantasypros.com/nfl/rankings/half-point-ppr-cheatsheets.php",
            ),
            (
                "ppr",
                "https://www.fantasypros.com/nfl/rankings/ppr-cheatsheets.php",
            ),
        ])
    }

    pub async fn scrape(&self) -> Result<(Vec<Player>, Vec<Rankings>)> {
        let mut all_players = Vec::new();
        let mut all_rankings = Vec::new();
        let mut seen_players = std::collections::HashSet::new();

        // Scrape each scoring type
        for (scoring_settings, url) in Self::get_urls() {
            self.tab.navigate_to(url)?;
            self.tab.wait_until_navigated()?;
            self.tab.wait_for_element("table#ranking-table")?;

            // Scroll to the last player
            // self.tab.evaluate(
            //     "let rows = document.querySelectorAll('tbody tr.player-row');
            //         let lastRow = rows[rows.length - 1];
            //         lastRow.scrollIntoView();",
            //     false,
            // )?;

            let ranking_table = self.tab.wait_for_element("table#ranking-table")?;
            let ranking_table_html =
                ranking_table.call_js_fn("function() { return this.outerHTML; }", vec![], false)?;
            let ranking_table_html_value = ranking_table_html
                .value
                .and_then(|v| v.as_str().map(|s| s.to_string()))
                .unwrap();

            let (players, rankings) = parse_rankings_html(
                &ranking_table_html_value,
                &mut seen_players,
                scoring_settings,
            )
            .await?;
            all_players.extend(players);
            all_rankings.extend(rankings);
        }

        Ok((all_players, all_rankings))
    }
}

async fn parse_rankings_html(
    table_html: &str,
    seen_players: &mut std::collections::HashSet<i32>,
    scoring_settings: &str,
) -> Result<(Vec<Player>, Vec<Rankings>)> {
    let document = Html::parse_document(table_html);
    let row_selector = Selector::parse("tbody tr.player-row").unwrap();
    let cell_selector = Selector::parse("td").unwrap();
    let ranking_regex = Regex::new(r"(\D+)(\d+)").unwrap();

    let mut players = Vec::new();
    let mut rankings = Vec::new();

    for row in document.select(&row_selector) {
        let cells: Vec<_> = row.select(&cell_selector).collect();

        let overall_ranking = cells[0].text().collect::<String>().parse::<i32>().ok();
        let player_identity = get_player_identity(&cells[2]);
        let (position, position_ranking) = get_position_ranking(&cells[3], &ranking_regex);
        let bye_week = cells[4].text().collect::<String>().parse::<i32>().ok();

        if let Some(player_id) = player_identity.id {
            if !seen_players.contains(&player_id) {
                let player_scraper = PlayerScraper::new(&player_identity.bio_url);
                let player_bio: PlayerBio = player_scraper.scrape().await?;

                players.push(Player {
                    id: player_identity.id,
                    name: player_identity.name,
                    position: position.clone(),
                    team: player_identity.team,
                    bye_week,
                    height: player_bio.height,
                    weight: player_bio.weight,
                    age: player_bio.age,
                    college: player_bio.college,
                });

                seen_players.insert(player_id);
            }

            rankings.push(Rankings {
                player_id,
                overall: overall_ranking,
                position: position_ranking.parse::<i32>().ok(),
                scoring_settings: scoring_settings.to_string(),
            });
        }
    }

    Ok((players, rankings))
}

fn get_player_identity(player_cell: &scraper::element_ref::ElementRef) -> PlayerIdentity {
    let player_id = player_cell
        .select(&Selector::parse("div").unwrap())
        .next()
        .unwrap()
        .value()
        .attr("data-player")
        .and_then(|s| s.parse::<i32>().ok());
    let team = player_cell
        .select(&Selector::parse("span").unwrap())
        .next()
        .unwrap()
        .text()
        .collect::<String>()
        .trim_matches(&['(', ')'][..])
        .to_string();
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
    poosition_cell: &scraper::element_ref::ElementRef,
    re: &Regex,
) -> (String, String) {
    let position_text = poosition_cell.text().collect::<String>();
    if let Some(caps) = re.captures(&position_text) {
        (caps[1].to_string(), caps[2].to_string())
    } else {
        (position_text, String::new())
    }
}
