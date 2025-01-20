<script lang="ts">
    import type { Rankings, Stats } from '$lib/types';
    import { Position } from '$lib/enums';
    import { POSITION_STATS } from '$lib/constants';

    type PositionType = typeof Position[keyof typeof Position];
    
    export let rankings: Rankings;
    export let stats: Stats;
    export let position: PositionType | null;

    $: positionStats = position ? POSITION_STATS[position] : [];
</script>

<style>
    .player-tables {
        height: 100%;
        margin-right: 1%;
        overflow: hidden;
    }

    .player-table {
        font-size: 0.75em;
        height: 50%;
        width: 100%;
    }

    .player-table table {
        background-color: #fcfcff;
    }

    .player-table tr td,
    .player-table tr th {
        padding-left: 1%;
    }

    .rank-table table {
        width: 35%;
    }

    .stats-table table {
        width: 100%;
    }

    .rank-header,
    .stats-header {
        margin-top: 0;
        margin-bottom: 0.2%;
        padding-top: 1.5%;
    }
</style>

<div class="player-tables">
    <div class="player-table">
        <h4 class="rank-header">Rankings</h4>
        <div class="rank-table">
            <table>
                <thead>
                    <tr>
                        <th>Overall</th>
                        <th>Position</th>
                        <th>Best</th>
                        <th>Worst</th>
                        <th>Average</th>
                        <th>Std Dev</th>
                    </tr>
                </thead>
                <tbody>
                    <tr>
                        <td>{rankings.overall}</td>
                        <td>{rankings.position}</td>
                        <td>{rankings.best}</td>
                        <td>{rankings.worst}</td>
                        <td>{rankings.average}</td>
                        <td>{rankings.standard_deviation}</td>
                    </tr>
                </tbody>
            </table>
        </div>
    </div>
    <div class="player-table">
        <h4 class="stats-header">Previous Stats</h4>
        <div class="stats-table">
            <table>
                <thead>
                    <tr>
                        {#each positionStats as stat}
                            <th>{stat.label}</th>
                        {/each}
                    </tr>
                </thead>
                <tbody>
                    <tr>
                        {#each positionStats as stat}
                            <td>{stats[stat.key]}</td>
                        {/each}
                    </tr>
                </tbody>
            </table>
        </div>
    </div>
</div> 