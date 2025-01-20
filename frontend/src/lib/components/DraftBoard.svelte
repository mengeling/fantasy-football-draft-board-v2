<script lang="ts">
    import { Team, Position } from '$lib/enums';
    import { defaultPlayer, type Player } from '$lib/types';

    export let players: Player[] = [];
    export let selectedPlayer: Player = defaultPlayer;

    let showAvailablePlayers = true;
    let positionFilter: Position = Position.ALL;
    let teamFilter: Team = Team.ALL;
    let playerNameSearch: string | null = null;

    function clearSearch() {
        positionFilter = Position.ALL;
        teamFilter = Team.ALL;
        playerNameSearch = null;
    }

    $: filteredPlayers = players.filter(player => {
        const matchesPosition = positionFilter === Position.ALL || player.position === positionFilter;
        const matchesTeam = teamFilter === Team.ALL || player.team === teamFilter;
        const matchesSearch = !playerNameSearch || 
            player.name.toLowerCase().includes(playerNameSearch.toLowerCase());
        const matchesAvailability = showAvailablePlayers ? !player.drafted : player.drafted;
        
        return matchesPosition && matchesTeam && matchesSearch && matchesAvailability;
    });

    $: {
        if (filteredPlayers.length === 0) {
            selectedPlayer = defaultPlayer;
        } else if (!filteredPlayers.includes(selectedPlayer)) {
            selectedPlayer = filteredPlayers[0];
        }
    }
</script>

<div class="board-container">
    <div class="available-drafted-container">
        <button 
            type="button"
            class="available-button" 
            class:active={showAvailablePlayers}
            on:click={() => showAvailablePlayers = true}
            on:keydown={(e) => e.key === 'Enter' && (showAvailablePlayers = true)}
        >
            Available Players
        </button>
        <button 
            type="button"
            class="drafted-button"
            class:active={!showAvailablePlayers}
            on:click={() => showAvailablePlayers = false}
            on:keydown={(e) => e.key === 'Enter' && (showAvailablePlayers = false)}
        >
            Drafted Players
        </button>
    </div>
    <div class="search-board-wrapper">
        <div class="position-team-player-search">
            <span class="position-text">Position:</span>
            <select class="position-dropdown" bind:value={positionFilter}>
                {#each Object.values(Position) as positionOption}
                    <option value={positionOption}>{positionOption}</option>
                {/each}
            </select>

            <span class="team-text">Team:</span>
            <select class="team-dropdown" bind:value={teamFilter}>
                {#each Object.values(Team) as teamOption}
                    <option value={teamOption}>{teamOption}</option>
                {/each}
            </select>

            <input 
                type="search" 
                placeholder="Player Name Search" 
                class="player-search"
                bind:value={playerNameSearch}
            >
            <button type="button" class="clear-search-button" on:click={clearSearch}>
                Clear Search
            </button>
        </div>
        <div class="table-wrapper">
            {#if filteredPlayers.length > 0}
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
                        {#each filteredPlayers as player, i}
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
