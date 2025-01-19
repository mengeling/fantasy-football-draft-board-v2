<script lang="ts">
    import { Team, Position } from '$lib/enums';
    import type { Player } from '$lib/types';

    export let players: Player[] = [];
    export let showAvailable = true;
    export let position: Position = Position.ALL;
    export let team: Team = Team.ALL;
    export let searchTerm = '';
    export let selectedPlayer: Player | undefined = undefined;

    function clearSearch() {
        position = Position.ALL;
        team = Team.ALL;
        searchTerm = '';
    }

    $: if (players.length > 0 && !selectedPlayer) {
        selectedPlayer = players[0];
    }
</script>

<div class="board-container">
    <div class="available-drafted-container">
        <button 
            type="button"
            class="available-button" 
            class:active={showAvailable}
            on:click={() => showAvailable = true}
            on:keydown={(e) => e.key === 'Enter' && (showAvailable = true)}
        >
            Available Players
        </button>
        <button 
            type="button"
            class="drafted-button"
            class:active={!showAvailable}
            on:click={() => showAvailable = false}
            on:keydown={(e) => e.key === 'Enter' && (showAvailable = false)}
        >
            Drafted Players
        </button>
    </div>
    <div class="search-board-wrapper">
        <div class="position-team-player-search">
            <span class="position-text">Position:</span>
            <select class="position-dropdown" bind:value={position}>
                {#each Object.values(Position) as positionOption}
                    <option value={positionOption}>{positionOption}</option>
                {/each}
            </select>

            <span class="team-text">Team:</span>
            <select class="team-dropdown" bind:value={team}>
                {#each Object.values(Team) as teamOption}
                    <option value={teamOption}>{teamOption}</option>
                {/each}
            </select>

            <input 
                type="search" 
                placeholder="Player Search" 
                class="player-search"
                bind:value={searchTerm}
            >
            <button type="button" class="clear-search-button" on:click={clearSearch}>
                Clear Search
            </button>
        </div>
        <div class="table-wrapper">
            {#if players.length > 0}
                <table class="draft-board">
                    <thead>
                        <tr>
                            <th>RANK</th>
                            <th>PLAYER</th>
                            <th>BYE</th>
                            <th>POS RANK</th>
                            <th>BEST</th>
                            <th>WORST</th>
                            <th>AVG</th>
                            <th>STDEV</th>
                            <th>PTS</th>
                            <th>PAC</th>
                            <th>PAYD</th>
                            <th>PATD</th>
                            <th>PAINT</th>
                            <th>RUSH</th>
                            <th>RUYD</th>
                            <th>RUTD</th>
                            <th>REC</th>
                            <th>REYD</th>
                            <th>RETD</th>
                        </tr>
                    </thead>
                    <tbody>
                        {#each players as player, i}
                            <tr
                                data-player-id={player.id}
                                on:click={() => selectedPlayer = player}
                                role="button"
                            >
                                <td>{player.rankings.overall}</td>
                                <td>{player.name}, {player.team}, {player.position}</td>
                                <td>{player.bye_week}</td>
                                <td>{player.rankings.position}</td>
                                <td>{player.rankings.best}</td>
                                <td>{player.rankings.worst}</td>
                                <td>{player.rankings.average}</td>
                                <td>{player.rankings.standard_deviation}</td>
                                <td>{player.stats.points?.toFixed(1)}</td>
                                <td>{player.stats.pass_cmp}</td>
                                <td>{player.stats.pass_yds}</td>
                                <td>{player.stats.pass_td}</td>
                                <td>{player.stats.pass_int}</td>
                                <td>{player.stats.rush_att}</td>
                                <td>{player.stats.rush_yds}</td>
                                <td>{player.stats.rush_td}</td>
                                <td>{player.stats.receptions}</td>
                                <td>{player.stats.rec_yds}</td>
                                <td>{player.stats.rec_td}</td>
                            </tr>
                        {/each}
                    </tbody>
                </table>
            {/if}
        </div>
    </div>
</div>
