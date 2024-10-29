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
