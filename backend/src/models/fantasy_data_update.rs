use serde::{Deserialize, Serialize};
use time::OffsetDateTime;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FantasyDataUpdate {
    pub id: i32,
    pub completed_at: OffsetDateTime,
}
