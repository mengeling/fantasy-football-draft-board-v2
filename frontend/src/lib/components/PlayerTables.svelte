<script lang="ts">
    import type { Rankings, Stats } from '$lib/types';
    import { Position } from '$lib/enums';
    import { POSITION_STATS } from '$lib/constants';
    import { defaultRankings, defaultStats } from '$lib/types';
    import { fmtNum, fmtPts } from '$lib/format';

    type PositionType = (typeof Position)[keyof typeof Position];

    export let rankings: Rankings;
    export let stats: Stats;
    export let position: PositionType | null;

    $: positionStats = position ? POSITION_STATS[position] : [];
    $: showTables = rankings !== defaultRankings && stats !== defaultStats;

    const rankFields: { key: keyof Rankings; label: string }[] = [
        { key: 'overall', label: 'Overall' },
        { key: 'position', label: 'Pos' },
        { key: 'best', label: 'Best' },
        { key: 'worst', label: 'Worst' },
        { key: 'average', label: 'Avg' },
        { key: 'standard_deviation', label: 'Std' }
    ];
</script>

<style>
    .stat-groups {
        flex: 1;
        min-width: 0;
        display: flex;
        flex-direction: column;
        gap: 10px;
        justify-content: flex-start;
    }

    .stat-group {
        background: var(--panel);
        border-radius: 12px;
        padding: 9px 16px;
    }

    .group-label {
        font-size: 0.62rem;
        text-transform: uppercase;
        letter-spacing: 0.08em;
        color: var(--text-muted);
        margin: 0 0 7px;
        font-weight: 600;
    }

    .stat-row {
        display: grid;
        gap: 10px 8px;
    }

    .stat {
        display: flex;
        flex-direction: column;
        align-items: flex-start;
        line-height: 1.15;
        min-width: 0;
    }

    .stat .v {
        font-size: 0.92rem;
        font-weight: 600;
        color: var(--text);
        font-variant-numeric: tabular-nums;
    }

    .stat .k {
        font-size: 0.56rem;
        text-transform: uppercase;
        letter-spacing: 0.05em;
        color: var(--text-muted);
        margin-top: 2px;
    }
</style>

{#if showTables}
    <div class="stat-groups">
        <div class="stat-group">
            <p class="group-label">Rankings</p>
            <div
                class="stat-row"
                style="grid-template-columns: repeat({Math.max(
                    rankFields.length,
                    positionStats.length
                )}, minmax(0, 1fr))"
            >
                {#each rankFields as field}
                    <div class="stat">
                        <span class="v">{fmtNum(rankings[field.key])}</span>
                        <span class="k">{field.label}</span>
                    </div>
                {/each}
            </div>
        </div>

        {#if positionStats.length}
            <div class="stat-group">
                <p class="group-label">Previous stats</p>
                <div
                    class="stat-row"
                    style="grid-template-columns: repeat({positionStats.length}, minmax(0, 1fr))"
                >
                    {#each positionStats as stat}
                        <div class="stat">
                            <span class="v"
                                >{stat.key === 'points'
                                    ? fmtPts(stats[stat.key])
                                    : fmtNum(stats[stat.key])}</span
                            >
                            <span class="k">{stat.label}</span>
                        </div>
                    {/each}
                </div>
            </div>
        {/if}
    </div>
{/if}
