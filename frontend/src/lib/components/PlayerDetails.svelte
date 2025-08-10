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
        margin-left: auto;
        margin-right: auto;
        margin-bottom: 2.5%;
        background-color: #e6f2ff;
        text-align: left;
        height: 160px;
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