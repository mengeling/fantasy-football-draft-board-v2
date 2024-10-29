use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Stats {
    pub player_id: i32,
    pub pass_cmp: f64,
    pub pass_att: f64,
    pub pass_cmp_pct: f64,
    pub pass_yds: f64,
    pub pass_yds_per_att: f64,
    pub pass_td: f64,
    pub pass_int: f64,
    pub pass_sacks: f64,
    pub rush_att: f64,
    pub rush_yds: f64,
    pub rush_yds_per_att: f64,
    pub rush_long: f64,
    pub rush_20: f64,
    pub rush_td: f64,
    pub fumbles: f64,
    pub receptions: f64,
    pub rec_tgt: f64,
    pub rec_yds: f64,
    pub rec_yds_per_rec: f64,
    pub rec_long: f64,
    pub rec_20: f64,
    pub rec_td: f64,
    pub field_goals: f64,
    pub fg_att: f64,
    pub fg_pct: f64,
    pub fg_long: f64,
    pub fg_1_19: f64,
    pub fg_20_29: f64,
    pub fg_30_39: f64,
    pub fg_40_49: f64,
    pub fg_50: f64,
    pub extra_points: f64,
    pub xp_att: f64,
    pub sacks: f64,
    pub int: f64,
    pub fumbles_recovered: f64,
    pub fumbles_forced: f64,
    pub def_td: f64,
    pub safeties: f64,
    pub special_teams_td: f64,
    pub games: f64,
    pub fantasy_pts: f64,
    pub fantasy_pts_per_game: f64,
}
