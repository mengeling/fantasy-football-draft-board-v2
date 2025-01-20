<script lang="ts">
	import type { Position, Team } from "$lib/enums";

    export let name: string;
    export let position: Position | null;
    export let team: Team | null;
    export let height: string;
    export let weight: string;
    export let age: number | null;
    export let college: string;
</script>

<style>
    .player-bio-container {
        height: 100%;
        width: 16.5%;
        float: left;
        padding-top: 10px;
        padding-bottom: 10px;
        box-sizing: border-box;
        overflow: hidden;
    }

    .player-name,
    .player-team-pos {
        margin-top: 0;
        margin-bottom: 2%;
    }

    .player-name {
        margin-top: 2%;
        margin-right: 2%;
    }

    .player-bio-sub {
        float: left;
        font-size: 0.8em;
        margin-right: 8%;
    }

    .player-detail-desc {
        font-weight: 600;
        margin-bottom: 0;
    }

    .player-detail-height,
    .player-detail-weight,
    .player-detail-age,
    .player-detail-college {
        margin: 0;
    }
</style>

<div class="player-bio-container">
    <h4 class="player-name">{name}</h4  >
    <h5 class="player-team-pos">{team} {position}</h5>
    <div class="player-bio-sub">
        <p class="player-detail-desc">Height</p>
        <p class="player-detail-height">{height}</p>
        <p class="player-detail-desc">Age</p>
        <p class="player-detail-age">{age}</p>
    </div>
    <div class="player-bio-sub">
        <p class="player-detail-desc">Weight</p>
        <p class="player-detail-weight">{weight}</p>
        <p class="player-detail-desc">College</p>
        <p class="player-detail-college">{college}</p>
    </div>
</div>
