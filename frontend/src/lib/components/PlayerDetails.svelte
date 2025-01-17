<script lang="ts">
    import { defaultPlayer, type Player } from '$lib/types';
    import PlayerImage from './PlayerImage.svelte';
    import PlayerBio from './PlayerBio.svelte';
    import PlayerTable from './PlayerTable.svelte';
    import { fetchApi } from '$lib/api';
    
    export let player: Player = defaultPlayer;
    export let onPlayerDraftChange: (player: Player) => void;

    async function handleDraftAction() {
        if (!player.id) return;

        const method = player.drafted ? 'DELETE' : 'POST';
        try {
            await fetchApi(`/drafted_players/${player.id}`, { method });
        } catch (error) {
            console.error('Failed to update draft status:', error);
            return;
        }

        const updatedPlayer = { ...player, drafted: !player.drafted };
        onPlayerDraftChange(updatedPlayer);
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
            class={player.drafted ? "drafted" : ""}
            on:click={handleDraftAction}
        >
            {player.drafted ? 'Undraft Selected Player' : 'Draft Selected Player'}
        </button>
    </div>
    
    <PlayerTable
        rankings={player.rankings}
        stats={player.stats}
        position={player.position}
    />
</div> 