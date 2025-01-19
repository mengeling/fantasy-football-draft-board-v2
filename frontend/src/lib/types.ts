import type { Position, Team } from './enums';

export type Rankings = {
	overall: number | null;
	position: number | null;
	best: number | null;
	worst: number | null;
	average: number | null;
	standardDeviation: number | null;
};

export const defaultRankings: Rankings = {
	overall: null,
	position: null,
	best: null,
	worst: null,
	average: null,
	standardDeviation: null
};

export type Stats = {
	passCmp: number | null;
	passAtt: number | null;
	passCmpPct: number | null;
	passYds: number | null;
	passYdsPerAtt: number | null;
	passTd: number | null;
	passInt: number | null;
	passSacks: number | null;
	rushAtt: number | null;
	rushYds: number | null;
	rushYdsPerAtt: number | null;
	rushLong: number | null;
	rush20: number | null;
	rushTd: number | null;
	fumbles: number | null;
	receptions: number | null;
	recTgt: number | null;
	recYds: number | null;
	recYdsPerRec: number | null;
	recLong: number | null;
	rec20: number | null;
	recTd: number | null;
	fieldGoals: number | null;
	fgAtt: number | null;
	fgPct: number | null;
	fgLong: number | null;
	fg119: number | null;
	fg2029: number | null;
	fg3039: number | null;
	fg4049: number | null;
	fg50: number | null;
	extraPoints: number | null;
	xpAtt: number | null;
	sacks: number | null;
	int: number | null;
	fumblesRecovered: number | null;
	fumblesForced: number | null;
	defTd: number | null;
	safeties: number | null;
	specialTeamsTd: number | null;
	games: number | null;
	points: number | null;
	pointsPerGame: number | null;
};

export const defaultStats: Stats = {
	passCmp: null,
	passAtt: null,
	passCmpPct: null,
	passYds: null,
	passYdsPerAtt: null,
	passTd: null,
	passInt: null,
	passSacks: null,
	rushAtt: null,
	rushYds: null,
	rushYdsPerAtt: null,
	rushLong: null,
	rush20: null,
	rushTd: null,
	fumbles: null,
	receptions: null,
	recTgt: null,
	recYds: null,
	recYdsPerRec: null,
	recLong: null,
	rec20: null,
	recTd: null,
	fieldGoals: null,
	fgAtt: null,
	fgPct: null,
	fgLong: null,
	fg119: null,
	fg2029: null,
	fg3039: null,
	fg4049: null,
	fg50: null,
	extraPoints: null,
	xpAtt: null,
	sacks: null,
	int: null,
	fumblesRecovered: null,
	fumblesForced: null,
	defTd: null,
	safeties: null,
	specialTeamsTd: null,
	games: null,
	points: null,
	pointsPerGame: null
};

export type Player = {
	id: number;
	name: string;
	position: Position | null;
	team: Team | null;
	byeWeek: number | null;
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
	byeWeek: null,
	height: '',
	weight: '',
	age: null,
	college: '',
	rankings: defaultRankings,
	stats: defaultStats,
	drafted: false
};
