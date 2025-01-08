-- Drop existing tables and types
DROP TABLE IF EXISTS rankings;
DROP TABLE IF EXISTS stats;
DROP TABLE IF EXISTS drafted_players;
DROP TABLE IF EXISTS users;
DROP TABLE IF EXISTS players;
DROP TABLE IF EXISTS fantasy_data_updates;

DROP TYPE IF EXISTS position_type;
DROP TYPE IF EXISTS team_type;
DROP TYPE IF EXISTS scoring_settings_type;

-- Create enum types
CREATE TYPE position_type AS ENUM (
    'QB', 'RB', 'WR', 'TE', 'K', 'DST'
);

CREATE TYPE team_type AS ENUM (
    'ARI', 'ATL', 'BAL', 'BUF', 'CAR', 'CHI', 'CIN', 'CLE',
    'DAL', 'DEN', 'DET', 'GB', 'HOU', 'IND', 'JAC', 'KC',
    'LV', 'LAC', 'LAR', 'MIA', 'MIN', 'NE', 'NO', 'NYG',
    'NYJ', 'PHI', 'PIT', 'SF', 'SEA', 'TB', 'TEN', 'WAS', 'FA'
);

CREATE TYPE scoring_settings_type AS ENUM (
    'Standard', 'Half', 'PPR'
);

-- Create tables
CREATE TABLE IF NOT EXISTS players (
    id INTEGER PRIMARY KEY,
    name TEXT NOT NULL,
    position position_type NOT NULL,
    team team_type NOT NULL,
    bye_week INTEGER,
    height TEXT NOT NULL,
    weight TEXT NOT NULL,
    age INTEGER,
    college TEXT NOT NULL          
);

CREATE TABLE IF NOT EXISTS rankings (
    player_id INTEGER,
    scoring_settings scoring_settings_type,
    overall INTEGER,
    position INTEGER,
    PRIMARY KEY (player_id, scoring_settings)
);

CREATE TABLE IF NOT EXISTS stats (
    player_id INTEGER PRIMARY KEY,
    pass_cmp DOUBLE PRECISION,
    pass_att DOUBLE PRECISION,
    pass_cmp_pct DOUBLE PRECISION,
    pass_yds DOUBLE PRECISION,
    pass_yds_per_att DOUBLE PRECISION,
    pass_td DOUBLE PRECISION,
    pass_int DOUBLE PRECISION,
    pass_sacks DOUBLE PRECISION,
    rush_att DOUBLE PRECISION,
    rush_yds DOUBLE PRECISION,
    rush_yds_per_att DOUBLE PRECISION,
    rush_long DOUBLE PRECISION,
    rush_20 DOUBLE PRECISION,
    rush_td DOUBLE PRECISION,
    fumbles DOUBLE PRECISION,
    receptions DOUBLE PRECISION,
    rec_tgt DOUBLE PRECISION,
    rec_yds DOUBLE PRECISION,
    rec_yds_per_rec DOUBLE PRECISION,
    rec_long DOUBLE PRECISION,
    rec_20 DOUBLE PRECISION,
    rec_td DOUBLE PRECISION,
    field_goals DOUBLE PRECISION,
    fg_att DOUBLE PRECISION,
    fg_pct DOUBLE PRECISION,
    fg_long DOUBLE PRECISION,
    fg_1_19 DOUBLE PRECISION,
    fg_20_29 DOUBLE PRECISION,
    fg_30_39 DOUBLE PRECISION,
    fg_40_49 DOUBLE PRECISION,
    fg_50 DOUBLE PRECISION,
    extra_points DOUBLE PRECISION,
    xp_att DOUBLE PRECISION,
    sacks DOUBLE PRECISION,
    int DOUBLE PRECISION,
    fumbles_recovered DOUBLE PRECISION,
    fumbles_forced DOUBLE PRECISION,
    def_td DOUBLE PRECISION,
    safeties DOUBLE PRECISION,
    special_teams_td DOUBLE PRECISION,
    games DOUBLE PRECISION,
    standard_pts DOUBLE PRECISION,
    standard_pts_per_game DOUBLE PRECISION,
    half_ppr_pts DOUBLE PRECISION,
    half_ppr_pts_per_game DOUBLE PRECISION,
    ppr_pts DOUBLE PRECISION,
    ppr_pts_per_game DOUBLE PRECISION
);

CREATE TABLE IF NOT EXISTS users (
    id SERIAL PRIMARY KEY,
    username VARCHAR(255) NOT NULL UNIQUE,
    scoring_settings scoring_settings_type,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP NOT NULL
);

CREATE TABLE IF NOT EXISTS drafted_players (
    id SERIAL PRIMARY KEY,
    user_id INTEGER NOT NULL,
    player_id INTEGER NOT NULL,
    drafted_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP NOT NULL,
    UNIQUE(user_id, player_id)
);

CREATE TABLE IF NOT EXISTS fantasy_data_updates (
    id SERIAL PRIMARY KEY,
    completed_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP NOT NULL
);
