use crate::models::rankings::ScoringSettings;
use serde::{Deserialize, Serialize};
use time::OffsetDateTime;

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub scoring_settings: ScoringSettings,
    pub created_at: OffsetDateTime,
}
