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
    pub id: Option<i32>,
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
    pub id: Option<i32>,
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
pub struct PlayerDenormalized {
    pub id: Option<i32>,
    pub name: String,
    pub position: Position,
    pub team: Team,
    pub bye_week: Option<i32>,
    pub height: String,
    pub weight: String,
    pub age: Option<i32>,
    pub college: String,
    pub overall_ranking: Option<i32>,
    pub position_ranking: Option<i32>,
    pub pass_cmp: Option<f64>,
    pub pass_att: Option<f64>,
    pub pass_cmp_pct: Option<f64>,
    pub pass_yds: Option<f64>,
    pub pass_yds_per_att: Option<f64>,
    pub pass_td: Option<f64>,
    pub pass_int: Option<f64>,
    pub pass_sacks: Option<f64>,
    pub rush_att: Option<f64>,
    pub rush_yds: Option<f64>,
    pub rush_yds_per_att: Option<f64>,
    pub rush_long: Option<f64>,
    pub rush_20: Option<f64>,
    pub rush_td: Option<f64>,
    pub fumbles: Option<f64>,
    pub receptions: Option<f64>,
    pub rec_tgt: Option<f64>,
    pub rec_yds: Option<f64>,
    pub rec_yds_per_rec: Option<f64>,
    pub rec_long: Option<f64>,
    pub rec_20: Option<f64>,
    pub rec_td: Option<f64>,
    pub field_goals: Option<f64>,
    pub fg_att: Option<f64>,
    pub fg_pct: Option<f64>,
    pub fg_long: Option<f64>,
    pub fg_1_19: Option<f64>,
    pub fg_20_29: Option<f64>,
    pub fg_30_39: Option<f64>,
    pub fg_40_49: Option<f64>,
    pub fg_50: Option<f64>,
    pub extra_points: Option<f64>,
    pub xp_att: Option<f64>,
    pub sacks: Option<f64>,
    pub int: Option<f64>,
    pub fumbles_recovered: Option<f64>,
    pub fumbles_forced: Option<f64>,
    pub def_td: Option<f64>,
    pub safeties: Option<f64>,
    pub special_teams_td: Option<f64>,
    pub games: Option<f64>,
    pub points: Option<f64>,
    pub points_per_game: Option<f64>,
}
