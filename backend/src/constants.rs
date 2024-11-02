use lazy_static::lazy_static;
use std::collections::HashMap;

lazy_static! {
    pub static ref STATS_HEADERS: HashMap<&'static str, Vec<&'static str>> = HashMap::from([
        (
            "qb",
            vec![
                "pass_cmp",
                "pass_att",
                "pass_cmp_pct",
                "pass_yds",
                "pass_yds_per_att",
                "pass_td",
                "pass_int",
                "pass_sacks",
                "rush_att",
                "rush_yds",
                "rush_td",
                "fumbles",
                "games",
                // "fantasy_pts",
                // "fantasy_pts_per_game",
            ]
        ),
        (
            "rb",
            vec![
                "rush_att",
                "rush_yds",
                "rush_yds_per_att",
                "rush_long",
                "rush_20",
                "rush_td",
                "receptions",
                "rec_tgt",
                "rec_yds",
                "rec_yds_per_rec",
                "rec_td",
                "fumbles",
                "games",
                // "fantasy_pts",
                // "fantasy_pts_per_game",
            ]
        ),
        (
            "wr",
            vec![
                "receptions",
                "rec_tgt",
                "rec_yds",
                "rec_yds_per_rec",
                "rec_long",
                "rec_20",
                "rec_td",
                "rush_att",
                "rush_yds",
                "rush_td",
                "fumbles",
                "games",
                // "fantasy_pts",
                // "fantasy_pts_per_game",
            ]
        ),
        (
            "te",
            vec![
                "receptions",
                "rec_tgt",
                "rec_yds",
                "rec_yds_per_rec",
                "rec_long",
                "rec_20",
                "rec_td",
                "rush_att",
                "rush_yds",
                "rush_td",
                "fumbles",
                "games",
                // "fantasy_pts",
                // "fantasy_pts_per_game",
            ]
        ),
        (
            "k",
            vec![
                "field_goals",
                "fg_att",
                "fg_pct",
                "fg_long",
                "fg_1_19",
                "fg_20_29",
                "fg_30_39",
                "fg_40_49",
                "fg_50",
                "extra_points",
                "xp_att",
                "games",
                // "fantasy_pts",
                // "fantasy_pts_per_game",
            ]
        ),
        (
            "dst",
            vec![
                "sacks",
                "int",
                "fumbles_recovered",
                "fumbles_forced",
                "def_td",
                "safeties",
                "special_teams_td",
                "games",
                // "fantasy_pts",
                // "fantasy_pts_per_game",
            ]
        ),
    ]);
}
