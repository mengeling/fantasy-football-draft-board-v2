use serde::{Deserialize, Serialize};
use time::OffsetDateTime;

#[derive(Debug, Serialize, Deserialize)]
pub struct DraftedPlayer {
    pub id: i32,
    pub user_id: i32,
    pub player_id: i32,
    pub drafted_at: OffsetDateTime,
}
