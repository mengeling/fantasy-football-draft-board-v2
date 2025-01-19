use crate::models::rankings::RankingsBase;
use crate::models::stats::StatsResponse;
use serde::{Deserialize, Serialize};
use sqlx::Type;
use strum::{Display, EnumIter, EnumString};

#[derive(Debug, Clone, Serialize, Deserialize, EnumString, Display, EnumIter, Type)]
#[sqlx(type_name = "position_type")]
pub enum Position {
    QB,
    RB,
    WR,
    TE,
    K,
    DST,
}

#[derive(Debug, Clone, Serialize, Deserialize, EnumString, Display, EnumIter, Type)]
#[sqlx(type_name = "team_type")]
pub enum Team {
    ARI,
    ATL,
    BAL,
    BUF,
    CAR,
    CHI,
    CIN,
    CLE,
    DAL,
    DEN,
    DET,
    GB,
    HOU,
    IND,
    JAC,
    KC,
    LV,
    LAC,
    LAR,
    MIA,
    MIN,
    NE,
    NO,
    NYG,
    NYJ,
    PHI,
    PIT,
    SF,
    SEA,
    TB,
    TEN,
    WAS,
    FA,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Player {
    pub id: i32,
    pub name: String,
    pub position: Position,
    pub team: Team,
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

#[derive(Debug, Clone)]
pub struct PlayerIdentity {
    pub id: i32,
    pub bio_url: String,
    pub name: String,
    pub team: Team,
}

#[derive(Debug, Clone)]
pub struct PlayerTask {
    pub player_id: i32,
    pub identity: PlayerIdentity,
    pub position: Position,
    pub bye_week: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlayerResponse {
    pub id: i32,
    pub name: String,
    pub position: Position,
    pub team: Team,
    pub bye_week: Option<i32>,
    pub height: String,
    pub weight: String,
    pub age: Option<i32>,
    pub college: String,
    pub rankings: RankingsBase,
    pub stats: StatsResponse,
    pub drafted: bool,
}

#[derive(Debug, Clone, Deserialize)]
pub struct PlayerQueryParameters {
    pub position: Option<Position>,
    pub team: Option<Team>,
    pub name: Option<String>,
}
