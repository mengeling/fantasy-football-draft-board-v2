<script lang="ts">
    export let refreshDate = '';
    export let onUpdateRankings: () => void;
    export let onLogout: () => void;
</script>

<div class="back-update-data">
    <button class="back-login-button" on:click={onLogout}>Back</button>
    <button type="button" class="update-rankings-button" on:click={onUpdateRankings}>
        Update Rankings
    </button>
    <p class="refresh-date">{refreshDate}</p>
</div> 