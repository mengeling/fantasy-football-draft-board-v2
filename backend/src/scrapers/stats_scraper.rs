use anyhow::Result;
use reqwest::Client;
use scraper::{Html, Selector};
use std::collections::HashMap;
use url::Url;

use crate::constants::{STATS_ALL_HEADERS, STATS_HEADERS};
use crate::models::player::Player;
use crate::utils::helpers::extract_player_id;

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

            let table_selector = Selector::parse("table#data tbody").unwrap();
            let row_selector = Selector::parse("tr").unwrap();
            let cell_selector = Selector::parse("td").unwrap();

            if let Some(table) = html.select(&table_selector).next() {
                for row in table.select(&row_selector) {
                    let player_id = extract_player_id(&row);
                    let mut stats = HashMap::new();

                    for (i, td) in row.select(&cell_selector).enumerate().skip(2) {
                        if i < headers.len() + 2 {
                            let value = td.text().collect::<String>().parse::<f64>().unwrap_or(0.0);
                            stats.insert(headers[i - 2].to_string(), Some(value));
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
                            image_url: String::new(),
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
