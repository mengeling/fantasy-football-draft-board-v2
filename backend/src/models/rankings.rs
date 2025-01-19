use serde::{Deserialize, Serialize};
use sqlx::Type;
use strum::{Display, EnumIter, EnumString};

#[derive(
    Debug, Clone, Serialize, Deserialize, Hash, Eq, PartialEq, EnumString, Display, EnumIter, Type,
)]
#[sqlx(type_name = "scoring_settings_type")]
pub enum ScoringSettings {
    Standard,
    Half,
    PPR,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RankingsBase {
    pub overall: i32,
    pub position: i32,
    pub best: i32,
    pub worst: i32,
    pub average: f32,
    pub standard_deviation: f32,
}

impl From<serde_json::Value> for RankingsBase {
    fn from(value: serde_json::Value) -> Self {
        serde_json::from_value(value).unwrap()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Rankings {
    pub player_id: i32,
    pub scoring_settings: ScoringSettings,
    #[serde(flatten)]
    pub base: RankingsBase,
}
