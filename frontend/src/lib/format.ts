// Display formatting for numeric stat/ranking values. Rounds floats to a
// single decimal (killing float artifacts like 383.90000000000003), adds
// thousands separators, and renders null/undefined as an em dash.
export function fmtNum(value: number | null | undefined): string {
	if (value === null || value === undefined) return '—';
	const rounded = Number.isInteger(value) ? value : Math.round(value * 10) / 10;
	return rounded.toLocaleString('en-US');
}

// Fantasy points: rounded to a whole number, with thousands separators.
export function fmtPts(value: number | null | undefined): string {
	if (value === null || value === undefined) return '—';
	return Math.round(value).toLocaleString('en-US');
}
