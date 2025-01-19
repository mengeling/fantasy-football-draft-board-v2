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

<div class="player-bio-container">
    <h3 class="player-name">{name}</h3>
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
