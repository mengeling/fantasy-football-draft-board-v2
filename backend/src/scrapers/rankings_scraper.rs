use std::collections::HashMap;

use anyhow::Result;
use headless_chrome::Tab;
use regex::Regex;
use scraper::{Html, Selector};

use crate::models::player::{Player, PlayerBio, Ranking};
use crate::scrapers::player_bio_scraper::PlayerBioScraper;
use crate::utils::helpers::{extract_player_data, extract_position_data};

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

        let table_element = self.tab.wait_for_element("table#ranking-table")?;
        let table_html_debug =
            table_element.call_js_fn("function() { return this.outerHTML; }", vec![], false)?;

        let table_html_value = table_html_debug.value.unwrap();
        let table_html = table_html_value.as_str().unwrap();

        let (players, rankings) = parse_rankings_html(&table_html).await?;
        // let bios = join_all(players.iter().map(|p| scrape_bio(p.bio_url.clone()))).await;
        Ok((players, rankings))
    }
}

async fn parse_rankings_html(table_html: &str) -> Result<(Vec<Player>, Vec<Ranking>)> {
    let document = Html::parse_document(table_html);
    let row_selector = Selector::parse("tbody tr.player-row").unwrap();
    let td_selector = Selector::parse("td").unwrap();
    let re = Regex::new(r"(\D+)(\d+)").unwrap();

    let mut players = Vec::new();
    let mut rankings = Vec::new();

    for row in document.select(&row_selector) {
        let tds: Vec<_> = row.select(&td_selector).collect();

        let overall_ranking = tds[0].text().collect::<String>().parse::<i32>().ok();
        let player_data = extract_player_data(&tds[2]);
        let (position, position_ranking) = extract_position_data(&tds[3], &re);
        let bye_week = tds[4].text().collect::<String>().parse::<i32>().ok();
        let player_bio_scraper = PlayerBioScraper::new(&player_data.bio_url);
        let bio: PlayerBio = player_bio_scraper.scrape().await?;

        players.push(Player {
            id: player_data.id,
            name: player_data.name,
            position,
            team: player_data.team,
            bye_week,
            height: bio.height,
            weight: bio.weight,
            age: bio.age,
            college: bio.college,
            stats: HashMap::new(),
        });

        rankings.push(Ranking {
            player_id: player_data.id.unwrap(),
            overall: overall_ranking,
            position: position_ranking.parse::<i32>().ok(),
        });
    }

    Ok((players, rankings))
}
