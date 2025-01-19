<script lang="ts">
    import type { Player } from '$lib/types';
    import PlayerImage from './PlayerImage.svelte';
    import PlayerBio from './PlayerBio.svelte';
    import PlayerTable from './PlayerTable.svelte';
    
    export let player: Player = {
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
        drafted: '',
        stats: ''
    };

    let isDrafted = false;

    function toggleDraft() {
        isDrafted = !isDrafted;
    }
</script>

<div class="player-details">
    <PlayerImage 
        id={player.id}
        name={player.name}
    />
    
    <PlayerBio 
        name={player.name}
        team={player.team}
        position={player.position}
        height={player.height}
        age={player.age}
        weight={player.weight}
        college={player.college}
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
    
    <PlayerTable
        rankings={player.rankings}
        stats={player.stats}
    />
</div> 