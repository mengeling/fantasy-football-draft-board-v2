import type { Position, Team } from './enums';

export type Rankings = {
	overall: number | null;
	position: number | null;
	best: number | null;
	worst: number | null;
	average: number | null;
	standard_deviation: number | null;
};

export const defaultRankings: Rankings = {
	overall: null,
	position: null,
	best: null,
	worst: null,
	average: null,
	standard_deviation: null
};

export type Stats = {
	[key: string]: number | null;
	pass_cmp: number | null;
	pass_att: number | null;
	pass_cmp_pct: number | null;
	pass_yds: number | null;
	pass_yds_per_att: number | null;
	pass_td: number | null;
	pass_int: number | null;
	pass_sacks: number | null;
	rush_att: number | null;
	rush_yds: number | null;
	rush_yds_per_att: number | null;
	rush_long: number | null;
	rush_20: number | null;
	rush_td: number | null;
	fumbles: number | null;
	receptions: number | null;
	rec_tgt: number | null;
	rec_yds: number | null;
	rec_yds_per_rec: number | null;
	rec_long: number | null;
	rec_20: number | null;
	rec_td: number | null;
	field_goals: number | null;
	fg_att: number | null;
	fg_pct: number | null;
	fg_long: number | null;
	fg_1_19: number | null;
	fg_20_29: number | null;
	fg_30_39: number | null;
	fg_40_49: number | null;
	fg_50: number | null;
	extra_points: number | null;
	xp_att: number | null;
	sacks: number | null;
	int: number | null;
	fumbles_recovered: number | null;
	fumbles_forced: number | null;
	def_td: number | null;
	safeties: number | null;
	special_teams_td: number | null;
	games: number | null;
	points: number | null;
	points_per_game: number | null;
};

export const defaultStats: Stats = {
	pass_cmp: null,
	pass_att: null,
	pass_cmp_pct: null,
	pass_yds: null,
	pass_yds_per_att: null,
	pass_td: null,
	pass_int: null,
	pass_sacks: null,
	rush_att: null,
	rush_yds: null,
	rush_yds_per_att: null,
	rush_long: null,
	rush_20: null,
	rush_td: null,
	fumbles: null,
	receptions: null,
	rec_tgt: null,
	rec_yds: null,
	rec_yds_per_rec: null,
	rec_long: null,
	rec_20: null,
	rec_td: null,
	field_goals: null,
	fg_att: null,
	fg_pct: null,
	fg_long: null,
	fg_1_19: null,
	fg_20_29: null,
	fg_30_39: null,
	fg_40_49: null,
	fg_50: null,
	extra_points: null,
	xp_att: null,
	sacks: null,
	int: null,
	fumbles_recovered: null,
	fumbles_forced: null,
	def_td: null,
	safeties: null,
	special_teams_td: null,
	games: null,
	points: null,
	points_per_game: null
};

export type Player = {
	id: number;
	name: string;
	position: Position | null;
	team: Team | null;
	bye_week: number | null;
	height: string;
	weight: string;
	age: number | null;
	college: string;
	rankings: Rankings;
	stats: Stats;
	drafted: boolean;
};

export const defaultPlayer: Player = {
	id: 0,
	name: '',
	position: null,
	team: null,
	bye_week: null,
	height: '',
	weight: '',
	age: null,
	college: '',
	rankings: defaultRankings,
	stats: defaultStats,
	drafted: false
};
