<script lang="ts">
    import { onMount } from 'svelte';
    import { fetchApi } from '$lib/api';
    
    let refreshDate = '';
    export let onLogout: () => void;

    async function fetchLastUpdate() {
        try {
            const data = await fetchApi('/fantasy-data/last-update');
            refreshDate = new Date(data.last_update).toLocaleString(undefined, {
                year: 'numeric',
                month: 'numeric',
                day: 'numeric'
            });
        } catch (error) {
            console.error('Error fetching last update time:', error);
        }
    }

    onMount(() => {
        fetchLastUpdate();
    });
</script>

<div class="back-update-data">
    <button class="back-login-button" on:click={onLogout}>Back</button>
    <p class="refresh-date">Rankings Date: {refreshDate}</p>
</div> 