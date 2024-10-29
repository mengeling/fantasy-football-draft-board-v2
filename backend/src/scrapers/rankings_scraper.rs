use std::collections::HashMap;

use anyhow::Result;
use headless_chrome::Tab;
use regex::Regex;
use scraper::{Html, Selector};

use crate::models::player::{Player, PlayerBio, PlayerIdentity, Ranking};
use crate::scrapers::player_scraper::PlayerScraper;

pub struct RankingsScraper<'a> {
    tab: &'a Tab,
    scoring: String,
}

impl<'a> RankingsScraper<'a> {
    pub fn new(tab: &'a Tab, scoring: &str) -> Self {
        Self {
            tab,
            scoring: scoring.to_string(),
        }
    }

    fn build_url(&self) -> String {
        match self.scoring.as_str() {
            "standard" => "https://www.fantasypros.com/nfl/rankings/consensus-cheatsheets.php",
            "half" => "https://www.fantasypros.com/nfl/rankings/half-point-ppr-cheatsheets.php",
            "ppr" => "https://www.fantasypros.com/nfl/rankings/ppr-cheatsheets.php",
            _ => "https://www.fantasypros.com/nfl/rankings/half-point-ppr-cheatsheets.php",
        }
        .to_string()
    }

    pub async fn scrape(&self) -> Result<(Vec<Player>, Vec<Ranking>)> {
        let url = self.build_url();
        self.tab.navigate_to(&url)?;
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

        let (players, rankings) = parse_rankings_html(&ranking_table_html_value).await?;
        Ok((players, rankings))
    }
}

async fn parse_rankings_html(table_html: &str) -> Result<(Vec<Player>, Vec<Ranking>)> {
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
        let (position, position_ranking) = extract_position_data(&cells[3], &ranking_regex);
        let bye_week = cells[4].text().collect::<String>().parse::<i32>().ok();
        let player_scraper = PlayerScraper::new(&player_identity.bio_url);
        let player_bio: PlayerBio = player_scraper.scrape().await?;

        players.push(Player {
            id: player_identity.id,
            name: player_identity.name,
            position,
            team: player_identity.team,
            bye_week,
            height: player_bio.height,
            weight: player_bio.weight,
            age: player_bio.age,
            college: player_bio.college,
            stats: HashMap::new(),
        });

        rankings.push(Ranking {
            player_id: player_identity.id.unwrap(),
            overall: overall_ranking,
            position: position_ranking.parse::<i32>().ok(),
        });
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

fn extract_position_data(td: &scraper::element_ref::ElementRef, re: &Regex) -> (String, String) {
    let text = td.text().collect::<String>();
    if let Some(caps) = re.captures(&text) {
        (caps[1].to_string(), caps[2].to_string())
    } else {
        (text, String::new())
    }
}
