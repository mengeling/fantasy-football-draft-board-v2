<script lang="ts">
    import LoginModal from '$lib/components/LoginModal.svelte';
    import Header from '$lib/components/Header.svelte';
    import PlayerDetails from '$lib/components/PlayerDetails.svelte';
    import DraftBoard from '$lib/components/DraftBoard.svelte';
    import ScoringModal from '$lib/components/ScoringModal.svelte';
    import { defaultPlayer, type Player, type User } from '$lib/types';
    import { fetchApi } from '$lib/api';
    import { ScoringSettings } from '$lib/enums';
    import { onMount } from 'svelte';

    let loggedIn = false;
    let players: Player[] = [];
    let selectedPlayer: Player = defaultPlayer;
    let showScoringModal = false;
    let currentUser: User | null = null;
    let loading = false;

    async function fetchPlayers() {
        try {
            players = await fetchApi('/players', { userId: currentUser?.id });
        } catch (e) {
            console.error('Error fetching players:', e);
        }
    }

    $: if (loggedIn) {
        fetchPlayers();
    }

    function handleLogin(user: User) {
        currentUser = user;
        loggedIn = true;
    }

    function handleLogout() {
        loggedIn = false;
        players = [];
        currentUser = null;
    }

    async function handleScoringUpdate(scoring: ScoringSettings) {
        if (!currentUser) return;
        
        loading = true;
        try {
            currentUser = await fetchApi(`/users/${currentUser.username}`, {
                method: 'PUT',
                body: JSON.stringify({ scoring_settings: scoring }),
                userId: currentUser.id
            });
            await fetchPlayers();
            showScoringModal = false;
        } catch (error) {
            console.error('Failed to update user:', error);
        } finally {
            loading = false;
        }
    }

    function handlePlayerDraftChange(updatedPlayer: Player) {
        const playerIndex = players.findIndex(p => p.id === updatedPlayer.id);
        if (playerIndex === -1) {
            throw new Error(`Failed to find player with ID ${updatedPlayer.id}`);
        }
        players = [
            ...players.slice(0, playerIndex),
            updatedPlayer,
            ...players.slice(playerIndex + 1)
        ];
    }

    onMount(() => {
        if (loggedIn) {
            fetchPlayers();
        }
    });
</script>

<main>
    <Header
        onLogout={handleLogout}
        onUpdateScoring={() => showScoringModal = true}
        {loading}
    />

    {#if !loggedIn}
        <LoginModal onLogin={handleLogin} />
    {/if}

    {#if showScoringModal}
        <ScoringModal
            onSelect={handleScoringUpdate}
            onCancel={() => showScoringModal = false}
        />
    {/if}

    <div class="main-content">
        <PlayerDetails
            player={selectedPlayer}
        />
        <DraftBoard 
            {players}
            bind:selectedPlayer
            onPlayerDraftChange={handlePlayerDraftChange}
            userId={currentUser?.id}
        />
    </div>
</main>
