<script lang="ts">
    import LoginModal from '$lib/components/LoginModal.svelte';
    import Header from '$lib/components/Header.svelte';
    import PlayerDetails from '$lib/components/PlayerDetails.svelte';
    import DraftBoard from '$lib/components/DraftBoard.svelte';
    import { defaultPlayer, type Player } from '$lib/types';
    import { fetchApi, setUserId, clearUserId } from '$lib/api';
    import { onMount } from 'svelte';

    let loggedIn = false;
    let players: Player[] = [];
    let selectedPlayer: Player = defaultPlayer;

    async function fetchPlayers() {
        try {
            players = await fetchApi('/players');
        } catch (e) {
            console.error('Error fetching players:', e);
        }
    }

    $: if (loggedIn) {
        fetchPlayers();
    }

    function handleLogin(username: string, userData: any) {
        loggedIn = true;
        setUserId(userData.id);
    }

    function handleLogout() {
        loggedIn = false;
        clearUserId();
        players = [];
    }

    onMount(() => {
        if (loggedIn) {
            fetchPlayers();
        }
    });
</script>

<main>
    {#if !loggedIn}
        <LoginModal onLogin={handleLogin} />
    {/if}

    <Header
        onLogout={handleLogout}
    />

    <div class="main-content">
        <PlayerDetails player={selectedPlayer} />
        <DraftBoard 
            {players}
            bind:selectedPlayer
        />
    </div>
</main>
