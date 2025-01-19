<script lang="ts">
    import type { Rankings, Stats } from '$lib/types';
    
    export let rankings: Rankings;
    export let stats: Stats;
</script>

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
                        <th>Points</th>
                        <th>Pass Comp</th>
                        <th>Pass Yds</th>
                        <th>Pass TD</th>
                        <th>Pass Int</th>
                        <th>Rush Att</th>
                        <th>Rush Yds</th>
                        <th>Rush TD</th>
                        <th>Receptions</th>
                        <th>Rec Yds</th>
                        <th>Rec TD</th>
                    </tr>
                </thead>
                <tbody>
                    <tr>
                        <td>{stats.points?.toFixed(1)}</td>
                        <td>{stats.pass_cmp}</td>
                        <td>{stats.pass_yds}</td>
                        <td>{stats.pass_td}</td>
                        <td>{stats.pass_int}</td>
                        <td>{stats.rush_att}</td>
                        <td>{stats.rush_yds}</td>
                        <td>{stats.rush_td}</td>
                        <td>{stats.receptions}</td>
                        <td>{stats.rec_yds}</td>
                        <td>{stats.rec_td}</td>
                    </tr>
                </tbody>
            </table>
        </div>
    </div>
</div> 