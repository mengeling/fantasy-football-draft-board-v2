use serde::Deserialize;

#[derive(Deserialize)]
pub struct DraftRequest {
    pub user_id: i32,
    pub player_id: i32,
}
