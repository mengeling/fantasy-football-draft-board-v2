use regex::Regex;
use scraper::Selector;
use std::collections::HashMap;

use crate::models::player::{Player, PlayerData};

pub fn extract_player_data(td: &scraper::element_ref::ElementRef) -> PlayerData {
    let div = td.select(&Selector::parse("div").unwrap()).next().unwrap();
    let a = td.select(&Selector::parse("a").unwrap()).next().unwrap();
    let span = td.select(&Selector::parse("span").unwrap()).next().unwrap();

    PlayerData {
        id: div
            .value()
            .attr("data-player")
            .and_then(|s| s.parse::<i32>().ok()),
        bio_url: a.value().attr("href").unwrap_or("").to_string(),
        name: a.text().collect::<String>(),
        team: span
            .text()
            .collect::<String>()
            .trim_matches(&['(', ')'][..])
            .to_string(),
    }
}

pub fn extract_position_data(
    td: &scraper::element_ref::ElementRef,
    re: &Regex,
) -> (String, String) {
    let text = td.text().collect::<String>();
    if let Some(caps) = re.captures(&text) {
        (caps[1].to_string(), caps[2].to_string())
    } else {
        (text, String::new())
    }
}

pub fn combine_player_data(rankings: Vec<Player>, stats: Vec<Player>) -> Vec<Player> {
    let mut combined = HashMap::new();

    for player in rankings {
        combined.insert(player.id, player);
    }

    for player in stats {
        if let Some(combined_player) = combined.get_mut(&player.id) {
            combined_player.stats = player.stats;
        }
    }

    combined.into_values().collect()
}

pub fn extract_player_id(row: &scraper::element_ref::ElementRef) -> Option<i32> {
    let class_name = row.value().attr("class").unwrap_or("");
    Regex::new(r"(\d+)")
        .unwrap()
        .captures(class_name)
        .and_then(|cap| cap.get(1))
        .and_then(|m| m.as_str().parse::<i32>().ok())
}
