<script lang="ts">
    import type { PlayerData } from '$lib/types';
    import PlayerImage from './PlayerImage.svelte';
    import PlayerBio from './PlayerBio.svelte';
    
    export let playerData: PlayerData = {
        id: '',
        name: '',
        team: '',
        position: '',
        height: '',
        age: '',
        weight: '',
        college: '',
        img_url: '',
        rankings: '',
        stats: ''
    };

    let isDrafted = false;

    function toggleDraft() {
        isDrafted = !isDrafted;
    }
</script>

<div class="player-details">
    <PlayerImage 
        id={playerData.id}
        imgUrl={playerData.img_url}
        name={playerData.name}
    />
    
    <PlayerBio 
        name={playerData.name}
        team={playerData.team}
        position={playerData.position}
        height={playerData.height}
        age={playerData.age}
        weight={playerData.weight}
        college={playerData.college}
    />
    
    <div class="draft-undraft-container">
        <button 
            type="button" 
            id="draft-undraft-button" 
            class={isDrafted ? "drafted" : ""}
            on:click={toggleDraft}
        >
            {isDrafted ? 'Undraft Selected Player' : 'Draft Selected Player'}
        </button>
    </div>
    
    <div class="player-tables">
        <div class="player-table">
            <h4 class="rank-header">Rankings</h4>
            <div class="rank-table">
                {@html playerData.rankings}
            </div>
        </div>
        <div class="player-table">
            <h4 class="stats-header">Previous Stats</h4>
            <div class="stats-table">
                {@html playerData.stats}
            </div>
        </div>
    </div>
</div> 