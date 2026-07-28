<script lang="ts">
    import { defaultPlayer, type Player } from '$lib/types';
    import PlayerImage from './PlayerImage.svelte';
    import PlayerBio from './PlayerBio.svelte';
    import PlayerTables from './PlayerTables.svelte';
    
    export let player: Player = defaultPlayer;

    $: showPlayerDetails = player !== defaultPlayer;
</script>

<style>
    .player-details {
        width: 98%;
        margin: 0 auto 1.1rem;
        background: var(--surface);
        border: 1px solid var(--border);
        border-radius: 14px;
        text-align: left;
        display: flex;
        align-items: flex-start;
        gap: 20px;
        padding: 16px 18px;
        min-height: 148px;
        box-sizing: border-box;
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
        <PlayerTables
            rankings={player.rankings}
            stats={player.stats}
            position={player.position}
        />
    {/if}
</div> 