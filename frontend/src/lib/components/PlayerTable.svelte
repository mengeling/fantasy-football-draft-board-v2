<script lang="ts">
    export let rankings: string;
    export let stats: string;
</script>

<div class="player-tables">
    <div class="player-table">
        <h4 class="rank-header">Rankings</h4>
        <div class="rank-table">
            {@html rankings}
        </div>
    </div>
    <div class="player-table">
        <h4 class="stats-header">Previous Stats</h4>
        <div class="stats-table">
            {@html stats}
        </div>
    </div>
</div> 