<script lang="ts">
    import { defaultPlayer, type Player } from '$lib/types';
    import PlayerImage from './PlayerImage.svelte';
    import PlayerBio from './PlayerBio.svelte';
    import PlayerTables from './PlayerTables.svelte';

    export let player: Player = defaultPlayer;
    let statsOpen = false;

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
        flex-wrap: wrap;
    }

    .stats-wrap {
        flex: 1;
        min-width: 0;
    }

    .stats-toggle {
        display: none;
        margin-left: auto;
        font-size: 0.72rem;
        padding: 5px 9px;
        color: var(--text-muted);
    }

    @media (max-width: 700px) {
        .player-details {
            width: 100%;
            gap: 12px;
            padding: 12px;
            min-height: 0;
            margin-bottom: 0.7rem;
            align-items: center;
        }

        .stats-toggle {
            display: inline-block;
            flex-shrink: 0;
            align-self: flex-start;
        }

        .stats-wrap {
            flex: 1 1 100%;
        }

        .stats-wrap:not(.open) {
            display: none;
        }
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
        <button
            type="button"
            class="stats-toggle"
            on:click={() => (statsOpen = !statsOpen)}
            aria-expanded={statsOpen}
        >
            Stats {statsOpen ? '▴' : '▾'}
        </button>
        <div class="stats-wrap" class:open={statsOpen}>
            <PlayerTables
                rankings={player.rankings}
                stats={player.stats}
                position={player.position}
            />
        </div>
    {/if}
</div>
