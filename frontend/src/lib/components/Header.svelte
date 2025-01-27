<script lang="ts">
    import { onMount } from 'svelte';
    import { fetchApi } from '$lib/api';
    
    let refreshDate = '';
    export let onLogout: () => void;
    export let onUpdateScoring: () => void;
    export let loading = false;

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

<style>
    .back-update-data {
        width: 98%;
        margin-left: auto;
        margin-right: auto;
        margin-top: 1.5%;
        margin-bottom: 1.2%;
        position: relative;
        display: flex;
        justify-content: space-between;
        align-items: center;
    }

    .back-login-button {
        font-size: 0.8em;
        padding: 0.5%;
    }

    .update-scoring-button {
        font-size: 0.8em;
        padding: 0.5%;
        position: absolute;
        left: 50%;
        transform: translateX(-50%);
    }

    .refresh-date {
        font-size: 0.8em;
        margin: 0;
    }
</style>

<div class="back-update-data">
    <button class="back-login-button" on:click={onLogout}>Back</button>
    <button 
        class="update-scoring-button" 
        on:click={onUpdateScoring}
        disabled={loading}
    >
        Update Scoring Settings
    </button>
    <p class="refresh-date">Rankings Date: {refreshDate}</p>
</div>
