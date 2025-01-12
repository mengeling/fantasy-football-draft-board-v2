<script lang="ts">
    import LoginModal from '$lib/components/LoginModal.svelte';
    import Header from '$lib/components/Header.svelte';
    import PlayerDetails from '$lib/components/PlayerDetails.svelte';
    import DraftBoard from '$lib/components/DraftBoard.svelte';
    import { Team, Position } from '$lib/enums';
    import type { Player } from '$lib/types';
    import { fetchApi, setUserId, clearUserId } from '$lib/api';
    import { onMount } from 'svelte';

    let loggedIn = false;
    let players: Player[] = [];
    let showAvailable = true;
    let position: Position = Position.ALL;
    let team: Team = Team.ALL;
    let searchTerm = '';
    
    let player = {
        id: '',
        name: '',
        team: '',
        position: '',
        height: '',
        age: '',
        weight: '',
        college: '',
        img_url: '',
        rankings: '',
        drafted: '',
        stats: ''
    };

    async function fetchPlayers() {
        try {
            const queryParams = new URLSearchParams({});
            if (position !== Position.ALL) {
                queryParams.append('position', position);
            }
            if (team !== Team.ALL) {
                queryParams.append('team', team);
            }
            if (searchTerm) {
                queryParams.append('name', searchTerm);
            }

            players = await fetchApi(`/players?${queryParams}`);
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
        <PlayerDetails {player} />
        <DraftBoard 
            {players}
            bind:showAvailable
            bind:position
            bind:team
            bind:searchTerm
        />
    </div>
</main>
