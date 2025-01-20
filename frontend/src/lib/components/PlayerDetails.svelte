<script lang="ts">
    import { defaultPlayer, type Player } from '$lib/types';
    import PlayerImage from './PlayerImage.svelte';
    import PlayerBio from './PlayerBio.svelte';
    import PlayerTables from './PlayerTables.svelte';
    import { fetchApi } from '$lib/api';
    
    export let player: Player = defaultPlayer;
    export let onPlayerDraftChange: (player: Player) => void;

    $: showPlayerDetails = player !== defaultPlayer;

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

<style>
    .player-details {
        width: 98%;
        margin-left: auto;
        margin-right: auto;
        margin-bottom: 2.5%;
        background-color: #e6f2ff;
        text-align: left;
        height: 160px;
    }

    .draft-undraft-container {
        height: 100%;
        width: 12%;
        display: flex;
        align-items: center;
        vertical-align: top;
        float: left;
    }

    #draft-undraft-button {
        font-size: 0.8em;
        padding: 3%;
        margin-top: 5%;
        margin-right: 35%;
        font-weight: 600;
        display: inline-block;
        vertical-align: middle;
    }
</style>

<div class="player-details">
    <PlayerImage 
        id={player.id}
        name={player.name}
    />
    
    {#if showPlayerDetails}
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
        
        <PlayerTables
            rankings={player.rankings}
            stats={player.stats}
            position={player.position}
        />
    {/if}
</div> 