<script>
    let showAvailable = true;
    let position = 'ALL';
    let team = 'ALL';
    let searchTerm = '';

    const teams = [
        'ARI', 'ATL', 'BAL', 'BUF', 'CAR', 'CHI', 'CIN', 'CLE', 'DAL', 'DEN', 
        'DET', 'FA', 'GB', 'HOU', 'IND', 'JAC', 'KC', 'LAC', 'LAR', 'MIA', 
        'MIN', 'NE', 'NO', 'NYG', 'NYJ', 'OAK', 'PHI', 'PIT', 'SEA', 'SF', 
        'TB', 'TEN', 'WAS'
    ];

    function clearSearch() {
        position = 'ALL';
        team = 'ALL';
        searchTerm = '';
    }
</script>

<div class="board-container">
    <div class="available-drafted-container">
        <span 
            class="available-button" 
            class:active={showAvailable}
            on:click={() => showAvailable = true}
        >
            Available Players
        </span>
        <span 
            class="drafted-button"
            class:active={!showAvailable}
            on:click={() => showAvailable = false}
        >
            Drafted Players
        </span>
    </div>
    <div class="search-board-wrapper">
        <div class="position-team-player-search">
            <span class="position-text">Position:</span>
            <select class="position-dropdown" bind:value={position}>
                <option value="ALL">ALL</option>
                <option value="QB">QB</option>
                <option value="RB">RB</option>
                <option value="WR">WR</option>
                <option value="TE">TE</option>
                <option value="K">K</option>
                <option value="DST">DST</option>
            </select>

            <span class="team-text">Team:</span>
            <select class="team-dropdown" bind:value={team}>
                <option value="ALL">ALL</option>
                {#each teams as teamOption}
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