use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct Player {
    pub id: Option<i32>,
    pub name: String,
    pub team: String,
    pub position: String,
    pub ranking: PlayerRanking,
    pub bye_week: Option<i32>,
    pub bio: PlayerBio,
    pub stats: HashMap<String, Option<f64>>,
    pub bio_url: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PlayerRanking {
    pub overall: Option<i32>,
    pub position: Option<i32>,
}

impl Default for PlayerRanking {
    fn default() -> Self {
        Self {
            overall: None,
            position: None,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PlayerBio {
    pub image_url: String,
    pub height: String,
    pub weight: String,
    pub age: Option<i32>,
    pub college: String,
}

impl Default for PlayerBio {
    fn default() -> Self {
        Self {
            image_url: String::new(),
            height: String::new(),
            weight: String::new(),
            age: None,
            college: String::new(),
        }
    }
}

pub struct PlayerData {
    pub id: Option<i32>,
    pub bio_url: String,
    pub name: String,
    pub team: String,
}
