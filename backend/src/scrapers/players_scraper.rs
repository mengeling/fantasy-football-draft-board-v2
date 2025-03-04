use anyhow::Result;
use futures::stream;
use futures::stream::StreamExt;
use reqwest::Client;
use scraper::{Html, Selector};
use std::collections::HashMap;

use crate::models::players::{Player, PlayerBio, PlayerTask};

pub struct PlayersScraper {
    client: Client,
    url: String,
}

impl PlayersScraper {
    pub fn new(url: &str) -> Self {
        PlayersScraper {
            client: Client::new(),
            url: url.to_string(),
        }
    }

    pub async fn scrape(&self) -> Result<PlayerBio> {
        let response = self.client.get(&self.url).send().await?;
        let body = response.text().await?;
        let html = Html::parse_document(&body);
        let bio_section_selector = Selector::parse("div.clearfix").unwrap();
        let bio_field_selector = Selector::parse("span.bio-detail").unwrap();

        let mut player_bio = PlayerBio {
            height: String::new(),
            weight: String::new(),
            age: None,
            college: String::new(),
            bye_week: None,
        };

        if let Some(bio_div) = html.select(&bio_section_selector).next() {
            let bio_details: HashMap<_, _> = bio_div
                .select(&bio_field_selector)
                .filter_map(|detail| {
                    let text = detail.text().collect::<String>();
                    let mut parts = text.split(": ");
                    Some((parts.next()?.to_string(), parts.next()?.to_string()))
                })
                .collect();

            player_bio.height = bio_details.get("Height").cloned().unwrap_or_default();
            player_bio.weight = bio_details.get("Weight").cloned().unwrap_or_default();
            player_bio.age = bio_details
                .get("Age")
                .and_then(|age| age.parse::<i32>().ok());
            player_bio.college = bio_details.get("College").cloned().unwrap_or_default();
        }

        let row_selector = Selector::parse("table.table-bordered:not(.sos) tbody tr").unwrap();
        let cell_selector = Selector::parse("table.table-bordered:not(.sos) td").unwrap();
        for (row_index, row) in html.select(&row_selector).enumerate() {
            let cells: Vec<_> = row.select(&cell_selector).collect();
            if cells.len() >= 2 {
                let opponent = cells[1].text().collect::<String>().trim().to_string();
                if opponent == "BYE" {
                    player_bio.bye_week = Some((row_index + 1) as i32);
                    break;
                }
            }
        }

        Ok(player_bio)
    }

    pub async fn process_tasks(tasks: Vec<PlayerTask>) -> Result<Vec<Player>> {
        let results: Vec<_> = stream::iter(tasks)
            .map(|task| {
                tokio::spawn(async move {
                    let player_scraper = PlayersScraper::new(&task.identity.bio_url);
                    let player_bio = player_scraper.scrape().await?;

                    Ok::<_, anyhow::Error>(Player {
                        id: task.identity.id,
                        name: task.identity.name,
                        position: task.position,
                        team: task.identity.team,
                        bye_week: player_bio.bye_week,
                        height: player_bio.height,
                        weight: player_bio.weight,
                        age: player_bio.age,
                        college: player_bio.college,
                    })
                })
            })
            .buffer_unordered(5)
            .collect()
            .await;

        let mut players = Vec::new();
        for result in results {
            match result {
                Ok(Ok(player)) => players.push(player),
                Ok(Err(e)) => println!("Error fetching player bio: {}", e),
                Err(e) => println!("Task join error: {}", e),
            }
        }

        Ok(players)
    }
}
