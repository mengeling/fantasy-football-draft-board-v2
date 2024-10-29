use anyhow::Result;
use reqwest::Client;
use scraper::{Html, Selector};
use std::collections::HashMap;

use crate::models::player::PlayerBio;

pub struct PlayerScraper {
    client: Client,
    url: String,
}

impl PlayerScraper {
    pub fn new(url: &str) -> Self {
        PlayerScraper {
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

        // NOTE: Don't need to scrape image_url anymore. Can now add player_id to url like this:
        // https://images.fantasypros.com/images/players/nfl/{PLAYER_ID}/headshot/70x70.png
        Ok(player_bio)
    }
}
