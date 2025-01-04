<script lang="ts">
    import { onMount } from 'svelte';
    import { ScoringType } from '$lib/types';
    import LoginModal from '$lib/components/LoginModal.svelte';
    import ScoringModal from '$lib/components/ScoringModal.svelte';
    import Header from '$lib/components/Header.svelte';
    import PlayerDetails from '$lib/components/PlayerDetails.svelte';
    import DraftBoard from '$lib/components/DraftBoard.svelte';

    let showLoginModal = true;
    let showScoringModal = false;
    let playerData = {
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
        stats: ''
    };
    let refreshDate = '';

    function handleLogin(username: string) {
        showLoginModal = false;
        showScoringModal = true;
    }

    function handleScoringSelect(scoring: ScoringType) {
        showScoringModal = false;
        // Add logic to create board with selected scoring
    }

    function handleUpdateRankings(scoring: ScoringType) {
        // Add logic to update rankings with selected scoring
        refreshDate = new Date().toLocaleString();
    }
</script>

<main>
    {#if showLoginModal}
        <LoginModal onLogin={handleLogin} />
    {/if}

    {#if showScoringModal}
        <ScoringModal 
            onSelect={handleScoringSelect}
            onCancel={() => showScoringModal = false}
        />
    {/if}

    <Header 
        {refreshDate}
        onUpdateRankings={handleUpdateRankings}
    />

    <div class="main-content">
        <PlayerDetails {playerData} />
        <DraftBoard />
    </div>
</main>
