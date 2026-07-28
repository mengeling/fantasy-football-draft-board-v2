<script lang="ts">
    import type { Position, Team } from '$lib/enums';

    export let name: string;
    export let position: Position | null;
    export let team: Team | null;
    export let height: string;
    export let weight: string;
    export let age: number | null;
    export let college: string;

    $: posClass = position ? `pos-${position.toString().toLowerCase()}` : '';
    $: bioLine = [
        height,
        weight,
        age ? `Age ${age}` : '',
        college
    ]
        .filter(Boolean)
        .join('  ·  ');
</script>

<style>
    .player-bio {
        width: 172px;
        flex-shrink: 0;
    }

    .player-name {
        font-size: 1.1rem;
        font-weight: 600;
        margin: 0 0 6px;
        line-height: 1.2;
    }

    .team-pos {
        display: flex;
        align-items: center;
        gap: 7px;
        margin-bottom: 10px;
    }

    .team {
        font-size: 0.82rem;
        color: var(--text-muted);
        font-weight: 500;
    }

    .bio-line {
        font-size: 0.76rem;
        color: var(--text-muted);
        line-height: 1.5;
        margin: 0;
    }
</style>

<div class="player-bio">
    <p class="player-name">{name}</p>
    <div class="team-pos">
        {#if position}
            <span class="pos-badge {posClass}">{position}</span>
        {/if}
        <span class="team">{team}</span>
    </div>
    <p class="bio-line">{bioLine || 'Bio unavailable'}</p>
</div>
