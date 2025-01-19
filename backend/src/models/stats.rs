use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct StatsBase {
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
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatsResponse {
    #[serde(flatten)]
    pub base: StatsBase,
    pub points: Option<f64>,
    pub points_per_game: Option<f64>,
}

impl From<serde_json::Value> for StatsResponse {
    fn from(value: serde_json::Value) -> Self {
        serde_json::from_value(value).unwrap()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Stats {
    pub player_id: i32,
    #[serde(flatten)]
    pub base: StatsBase,
    pub standard_pts: f64,
    pub standard_pts_per_game: f64,
    pub half_ppr_pts: f64,
    pub half_ppr_pts_per_game: f64,
    pub ppr_pts: f64,
    pub ppr_pts_per_game: f64,
}

impl Stats {
    pub fn new(player_id: i32) -> Self {
        Self {
            player_id,
            ..Default::default()
        }
    }

    pub fn update_from(&mut self, other: &Stats) {
        macro_rules! update_max_base {
            ($($field:ident),*) => {
                $(self.base.$field = self.base.$field.max(other.base.$field);)*
            }
        }

        macro_rules! update_max_scoring {
            ($($field:ident),*) => {
                $(self.$field = self.$field.max(other.$field);)*
            }
        }

        update_max_base!(
            pass_cmp,
            pass_att,
            pass_cmp_pct,
            pass_yds,
            pass_yds_per_att,
            pass_td,
            pass_int,
            pass_sacks,
            rush_att,
            rush_yds,
            rush_yds_per_att,
            rush_long,
            rush_20,
            rush_td,
            fumbles,
            receptions,
            rec_tgt,
            rec_yds,
            rec_yds_per_rec,
            rec_long,
            rec_20,
            rec_td,
            field_goals,
            fg_att,
            fg_pct,
            fg_long,
            fg_1_19,
            fg_20_29,
            fg_30_39,
            fg_40_49,
            fg_50,
            extra_points,
            xp_att,
            sacks,
            int,
            fumbles_recovered,
            fumbles_forced,
            def_td,
            safeties,
            special_teams_td,
            games
        );

        update_max_scoring!(
            standard_pts,
            standard_pts_per_game,
            half_ppr_pts,
            half_ppr_pts_per_game,
            ppr_pts,
            ppr_pts_per_game
        );
    }
}
