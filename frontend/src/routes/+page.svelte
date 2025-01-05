<script lang="ts">
    import LoginModal from '$lib/components/LoginModal.svelte';
    import Header from '$lib/components/Header.svelte';
    import PlayerDetails from '$lib/components/PlayerDetails.svelte';
    import DraftBoard from '$lib/components/DraftBoard.svelte';

    let showLoginModal = true;
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

    function handleLogin(username: string, userData: any) {
        showLoginModal = false;
    }

    function handleUpdateRankings() {
        refreshDate = new Date().toLocaleString();
    }

    function handleLogout() {
        showLoginModal = true;
    }
</script>

<main>
    {#if showLoginModal}
        <LoginModal onLogin={handleLogin} />
    {/if}

    <Header
        {refreshDate}
        onLogout={handleLogout}
    />

    <div class="main-content">
        <PlayerDetails {playerData} />
        <DraftBoard />
    </div>
</main>
