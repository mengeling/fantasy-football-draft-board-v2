<script lang="ts">
    export let name: string;
    export let team: string;
    export let position: string;
    export let height: string;
    export let age: string;
    export let weight: string;
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
