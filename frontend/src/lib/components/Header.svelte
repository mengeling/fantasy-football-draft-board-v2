<script lang="ts">
    export let refreshDate = '';
    export let onLogout: () => void;
</script>

<div class="back-update-data">
    <button class="back-login-button" on:click={onLogout}>Back</button>
    <p class="refresh-date">{refreshDate}</p>
</div> 