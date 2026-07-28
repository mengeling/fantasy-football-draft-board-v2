<script lang="ts">
    import LoginModal from '$lib/components/LoginModal.svelte';
    import Header from '$lib/components/Header.svelte';
    import PlayerDetails from '$lib/components/PlayerDetails.svelte';
    import DraftBoard from '$lib/components/DraftBoard.svelte';
    import ScoringModal from '$lib/components/ScoringModal.svelte';
    import ResetBoardModal from '$lib/components/ResetBoardModal.svelte';
    import { defaultPlayer, type Player, type User } from '$lib/types';
    import { fetchApi } from '$lib/api';
    import { ScoringSettings } from '$lib/enums';
    import { onMount } from 'svelte';

    let loggedIn = false;
    let players: Player[] = [];
    let selectedPlayer: Player = defaultPlayer;
    let showScoringModal = false;
    let showResetModal = false;
    let currentUser: User | null = null;
    let loading = false;

    $: draftedCount = players.filter(p => p.drafted).length;

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
            currentUser = await fetchApi(`/users/${encodeURIComponent(currentUser.username)}`, {
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

    async function handleResetBoard() {
        if (!currentUser) return;

        loading = true;
        try {
            await fetchApi('/drafted_players', { method: 'DELETE', userId: currentUser.id });
            players = players.map(p => (p.drafted ? { ...p, drafted: false } : p));
            selectedPlayer = defaultPlayer;
            showResetModal = false;
        } catch (error) {
            console.error('Failed to reset board: ', error);
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
        username={currentUser?.username ?? ''}
        onLogout={handleLogout}
        onUpdateScoring={() => showScoringModal = true}
        onResetBoard={() => showResetModal = true}
        {draftedCount}
        {loading}
    />

    {#if !loggedIn}
        <LoginModal onLogin={handleLogin} />
    {/if}

    {#if showScoringModal}
        <ScoringModal
            prompt="This sets how points are scored for your board."
            onSelect={handleScoringUpdate}
            onCancel={() => showScoringModal = false}
        />
    {/if}

    {#if showResetModal}
        <ResetBoardModal
            {draftedCount}
            onConfirm={handleResetBoard}
            onCancel={() => showResetModal = false}
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
