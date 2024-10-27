use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Player {
    pub id: Option<i32>,
    pub name: String,
    pub position: String,
    pub team: String,
    pub bye_week: Option<i32>,
    pub image_url: String,
    pub height: String,
    pub weight: String,
    pub age: Option<i32>,
    pub college: String,
    pub stats: HashMap<String, Option<f64>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlayerBio {
    pub image_url: String,
    pub height: String,
    pub weight: String,
    pub age: Option<i32>,
    pub college: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Ranking {
    pub player_id: i32,
    pub overall: Option<i32>,
    pub position: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Stat {
    pub player_id: i32,
    pub stat_name: String,
    pub stat_value: f64,
}

//  REMOVE
pub struct PlayerData {
    pub id: Option<i32>,
    pub bio_url: String,
    pub name: String,
    pub team: String,
}
