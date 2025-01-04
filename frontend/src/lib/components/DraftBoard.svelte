<script>
    import { Team, Position } from '$lib/types';
    let showAvailable = true;
    let position = Position.ALL;
    let team = Team.ALL;
    let searchTerm = '';

    function clearSearch() {
        position = Position.ALL;
        team = Team.ALL;
        searchTerm = '';
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
            <div class="draft-board">
                <!-- Draft board content will be dynamically populated -->
            </div>
        </div>
    </div>
</div> 