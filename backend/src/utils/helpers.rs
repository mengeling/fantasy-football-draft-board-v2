use regex::Regex;
use scraper::Selector;
use std::collections::HashMap;

use crate::models::player::Player;

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
