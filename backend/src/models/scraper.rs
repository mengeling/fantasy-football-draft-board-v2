use serde::{Deserialize, Serialize};
use time::OffsetDateTime;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScraperRuns {
    pub id: i32,
    pub completed_at: OffsetDateTime,
}
