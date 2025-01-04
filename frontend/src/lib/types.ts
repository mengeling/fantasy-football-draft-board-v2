export enum ScoringSettings {
	Standard,
	Half,
	PPR
}

export enum Team {
	ALL,
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
	FA,
	GB,
	HOU,
	IND,
	JAC,
	KC,
	LAC,
	LAR,
	LV,
	MIA,
	MIN,
	NE,
	NO,
	NYG,
	NYJ,
	PHI,
	PIT,
	SEA,
	SF,
	TB,
	TEN,
	WAS
}

export enum Position {
	ALL,
	QB,
	RB,
	WR,
	TE,
	K,
	DST
}

export interface PlayerData {
	id: string;
	name: string;
	team: string;
	position: string;
	height: string;
	age: string;
	weight: string;
	college: string;
	img_url: string;
	rankings: string;
	stats: string;
}
