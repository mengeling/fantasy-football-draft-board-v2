use anyhow::Result;
use regex::Regex;
use reqwest::Client;
use scraper::{Html, Selector};
use std::collections::HashMap;
use url::Url;

use crate::constants::{STATS_ALL_HEADERS, STATS_HEADERS};
use crate::models::player::Player;

pub struct StatsScraper {
    client: Client,
    scoring: String,
}

impl StatsScraper {
    pub fn new(scoring: &str) -> Self {
        StatsScraper {
            client: Client::new(),
            scoring: scoring.to_string(),
        }
    }

    fn build_url(&self, position: &str) -> Result<String> {
        let base_url = "https://www.fantasypros.com/nfl/stats/";
        let mut url = Url::parse(base_url)?;
        url.path_segments_mut()
            .map_err(|_| anyhow::anyhow!("Cannot modify URL"))?
            .push(position)
            .push("");

        if self.scoring != "standard" {
            url.query_pairs_mut().append_pair(
                "scoring",
                match self.scoring.as_str() {
                    "half" => "HALF",
                    "ppr" => "PPR",
                    _ => "HALF",
                },
            );
        }

        Ok(url.to_string())
    }

    pub async fn scrape(&self) -> Result<Vec<Player>> {
        let mut players: Vec<Player> = Vec::new();

        for (position, headers) in STATS_HEADERS.iter() {
            let url = self.build_url(position)?;
            let response = self.client.get(url).send().await?;
            let html = Html::parse_document(&response.text().await?);

            let stats_table_selector = Selector::parse("table#data tbody").unwrap();
            let stats_row_selector = Selector::parse("tr").unwrap();
            let stats_cell_selector = Selector::parse("td").unwrap();

            if let Some(stats_table) = html.select(&stats_table_selector).next() {
                for row in stats_table.select(&stats_row_selector) {
                    let player_id = get_player_id(&row);
                    let mut stats = HashMap::new();

                    for (cell_index, cell) in row.select(&stats_cell_selector).enumerate().skip(2) {
                        if cell_index < headers.len() + 2 {
                            let value = cell
                                .text()
                                .collect::<String>()
                                .parse::<f64>()
                                .unwrap_or(0.0);
                            stats.insert(headers[cell_index - 2].to_string(), Some(value));
                        }
                    }

                    if let Some(existing_player) = players.iter_mut().find(|p| p.id == player_id) {
                        // Update existing player's stats only if new value is greater
                        for (key, &value) in &stats {
                            existing_player
                                .stats
                                .entry(key.clone())
                                .and_modify(|e| {
                                    *e = Some(e.unwrap_or(0.0).max(value.unwrap_or(0.0)))
                                })
                                .or_insert(value);
                        }
                    } else {
                        // Create a new player with all stats initialized to 0.0
                        let mut all_stats = HashMap::new();
                        for &header in STATS_ALL_HEADERS.iter() {
                            all_stats.insert(header.to_string(), Some(0.0));
                        }
                        // Update with the current position's stats
                        all_stats.extend(stats);

                        players.push(Player {
                            id: player_id,
                            name: String::new(),
                            position: String::new(),
                            team: String::new(),
                            bye_week: None,
                            height: String::new(),
                            weight: String::new(),
                            age: None,
                            college: String::new(),
                            stats: all_stats,
                        });
                    }
                }
            }
        }

        Ok(players)
    }
}

pub fn get_player_id(row: &scraper::element_ref::ElementRef) -> Option<i32> {
    let row_class = row.value().attr("class").unwrap_or("");
    Regex::new(r"(\d+)")
        .unwrap()
        .captures(row_class)
        .and_then(|cap| cap.get(1))
        .and_then(|m| m.as_str().parse::<i32>().ok())
}
