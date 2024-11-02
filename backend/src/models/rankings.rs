use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Rankings {
    pub player_id: i32,
    pub scoring_settings: String,
    pub overall: Option<i32>,
    pub position: Option<i32>,
}
