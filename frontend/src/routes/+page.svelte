<script lang="ts">
    import { onMount } from 'svelte';
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

    // Add your state management and event handlers here
</script>

<main>
    {#if showLoginModal}
        <LoginModal 
            on:login={(e) => {/* handle login */}} 
        />
    {/if}

    {#if showScoringModal}
        <ScoringModal 
            on:select={(e) => {/* handle scoring selection */}}
            on:cancel={() => showScoringModal = false}
        />
    {/if}

    <Header 
        {refreshDate}
        on:updateRankings={() => {/* handle rankings update */}}
    />

    <div class="main-content">
        <PlayerDetails {playerData} />
        <DraftBoard />
    </div>
</main>
