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
pub struct Rankings {
    pub player_id: i32,
    pub scoring_settings: ScoringSettings,
    pub overall: Option<i32>,
    pub position: Option<i32>,
}
