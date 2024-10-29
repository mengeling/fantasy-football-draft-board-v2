use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Rankings {
    pub player_id: i32,
    pub overall: Option<i32>,
    pub position: Option<i32>,
}
