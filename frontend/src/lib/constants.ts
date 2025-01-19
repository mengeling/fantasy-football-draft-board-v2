import { Position } from './enums';

type StatHeader = {
	key: string;
	label: string;
};

export const POSITION_STATS: Record<Position, StatHeader[]> = {
	[Position.QB]: [
		{ key: 'points', label: 'PTS' },
		{ key: 'games', label: 'G' },
		{ key: 'pass_cmp', label: 'PC' },
		{ key: 'pass_att', label: 'PA' },
		{ key: 'pass_cmp_pct', label: 'PCP' },
		{ key: 'pass_yds', label: 'PYD' },
		{ key: 'pass_yds_per_att', label: 'YPA' },
		{ key: 'pass_td', label: 'PTD' },
		{ key: 'pass_int', label: 'PINT' },
		{ key: 'rush_att', label: 'RUSH' },
		{ key: 'rush_yds', label: 'RUYD' },
		{ key: 'rush_td', label: 'RUTD' },
		{ key: 'fumbles', label: 'FUM' }
	],
	[Position.RB]: [
		{ key: 'points', label: 'PTS' },
		{ key: 'games', label: 'G' },
		{ key: 'rush_att', label: 'RUSH' },
		{ key: 'rush_yds', label: 'RUYD' },
		{ key: 'rush_yds_per_att', label: 'YPA' },
		{ key: 'rush_20', label: '20+' },
		{ key: 'rush_td', label: 'RUTD' },
		{ key: 'receptions', label: 'REC' },
		{ key: 'rec_tgt', label: 'RETG' },
		{ key: 'rec_yds', label: 'REYD' },
		{ key: 'rec_yds_per_rec', label: 'YPR' },
		{ key: 'rec_td', label: 'RETD' },
		{ key: 'fumbles', label: 'FUM' }
	],
	[Position.WR]: [
		{ key: 'points', label: 'PTS' },
		{ key: 'games', label: 'G' },
		{ key: 'receptions', label: 'REC' },
		{ key: 'rec_tgt', label: 'RETG' },
		{ key: 'rec_yds', label: 'REYD' },
		{ key: 'rec_yds_per_rec', label: 'YPR' },
		{ key: 'rec_20', label: 'RE20' },
		{ key: 'rec_td', label: 'RETD' },
		{ key: 'rush_att', label: 'RUSH' },
		{ key: 'rush_yds', label: 'RUYD' },
		{ key: 'rush_td', label: 'RUTD' },
		{ key: 'fumbles', label: 'FUM' }
	],
	[Position.TE]: [
		{ key: 'points', label: 'PTS' },
		{ key: 'games', label: 'G' },
		{ key: 'receptions', label: 'REC' },
		{ key: 'rec_tgt', label: 'RETG' },
		{ key: 'rec_yds', label: 'REYD' },
		{ key: 'rec_yds_per_rec', label: 'YPR' },
		{ key: 'rec_20', label: 'RE20' },
		{ key: 'rec_td', label: 'RETD' },
		{ key: 'rush_att', label: 'RUSH' },
		{ key: 'rush_yds', label: 'RUYD' },
		{ key: 'rush_td', label: 'RUTD' },
		{ key: 'fumbles', label: 'FUM' }
	],
	[Position.K]: [
		{ key: 'points', label: 'PTS' },
		{ key: 'games', label: 'G' },
		{ key: 'field_goals', label: 'FGM' },
		{ key: 'fg_att', label: 'FGA' },
		{ key: 'fg_pct', label: 'FGP' },
		{ key: 'fg_long', label: 'LONG' },
		{ key: 'fg_1_19', label: '1-19' },
		{ key: 'fg_20_29', label: '20-29' },
		{ key: 'fg_30_39', label: '30-39' },
		{ key: 'fg_40_49', label: '40-49' },
		{ key: 'fg_50', label: '50+' },
		{ key: 'extra_points', label: 'EPM' },
		{ key: 'xp_att', label: 'EPA' }
	],
	[Position.DST]: [
		{ key: 'points', label: 'PTS' },
		{ key: 'games', label: 'G' },
		{ key: 'sacks', label: 'SACK' },
		{ key: 'int', label: 'INT' },
		{ key: 'fumbles_recovered', label: 'FR' },
		{ key: 'fumbles_forced', label: 'FF' },
		{ key: 'def_td', label: 'DTD' },
		{ key: 'safeties', label: 'SAFETY' },
		{ key: 'special_teams_td', label: 'STTD' }
	],
	[Position.ALL]: []
};
