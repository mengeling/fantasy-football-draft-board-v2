use anyhow::Result;
use async_trait::async_trait;
use headless_chrome::Tab;
use scraper::{Html, Selector};
use regex::Regex;
use futures::future::join_all;
use std::collections::HashMap;

use crate::models::player::{Player, PlayerBio, PlayerRanking, PlayerData};
use crate::scrapers::Scraper;
use crate::utils::helpers::{extract_player_data, extract_position_data, scrape_bio};

pub struct RankingsScraper<'a> {
    tab: &'a Tab,
    url: String,
}

impl<'a> RankingsScraper<'a> {
    pub fn new(tab: &'a Tab, url: String) -> Self {
        Self { tab, url }
    }
}

#[async_trait]
impl<'a> Scraper for RankingsScraper<'a> {
    async fn scrape(&self) -> Result<Vec<Player>> {
        self.tab.navigate_to(&self.url)?;
        self.tab.wait_until_navigated()?;
        self.tab.wait_for_element("table#ranking-table")?;

        let table_element = self.tab.wait_for_element("table#ranking-table")?;
        let table_html_debug =
            table_element.call_js_fn("function() { return this.outerHTML; }", vec![], false)?;

        let table_html_value = table_html_debug.value.unwrap();
        let table_html = table_html_value.as_str().unwrap();

        let players = parse_rankings_html(&table_html)?;
        let bios = join_all(players.iter().map(|p| scrape_bio(p.bio_url.clone()))).await;
        Ok(players
            .into_iter()
            .zip(bios)
            .map(|(mut player, bio)| {
                player.bio = bio.unwrap_or_default();
                player
            })
            .collect())
    }
}

fn parse_rankings_html(table_html: &str) -> Result<Vec<Player>> {
    let document = Html::parse_document(table_html);
    let row_selector = Selector::parse("tbody tr.player-row").unwrap();
    let td_selector = Selector::parse("td").unwrap();
    let re = Regex::new(r"(\D+)(\d+)").unwrap();

    let mut players = Vec::new();

    for row in document.select(&row_selector) {
        let tds: Vec<_> = row.select(&td_selector).collect();

        let overall_ranking = tds[0].text().collect::<String>().parse::<i32>().ok();
        let player_data = extract_player_data(&tds[2]);
        let (position, position_ranking) = extract_position_data(&tds[3], &re);
        let bye_week = tds[4].text().collect::<String>().parse::<i32>().ok();

        players.push(Player {
            id: player_data.id,
            name: player_data.name,
            team: player_data.team,
            position,
            ranking: PlayerRanking {
                overall: overall_ranking,
                position: position_ranking.parse::<i32>().ok(),
            },
            bye_week,
            bio: PlayerBio::default(),
            stats: HashMap::new(),
            bio_url: player_data.bio_url,
        });
    }

    Ok(players)
}
