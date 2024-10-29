use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Player {
    pub id: Option<i32>,
    pub name: String,
    pub position: String,
    pub team: String,
    pub bye_week: Option<i32>,
    pub height: String,
    pub weight: String,
    pub age: Option<i32>,
    pub college: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlayerBio {
    pub height: String,
    pub weight: String,
    pub age: Option<i32>,
    pub college: String,
}

//  REMOVE
pub struct PlayerIdentity {
    pub id: Option<i32>,
    pub bio_url: String,
    pub name: String,
    pub team: String,
}
