<script lang="ts">
    import { ScoringType } from '$lib/types';
    
    export let refreshDate = '';
    export let onUpdateRankings: (scoring: ScoringType) => void;
    
    let showPopup = false;
    let loading = false;

    function handleUpdateRankings() {
        showPopup = true;
    }

    function handleScoringSelect(scoring: ScoringType) {
        loading = true;
        showPopup = false;
        onUpdateRankings(scoring);
    }
</script>

<div class="back-update-data">
    <button class="back-login-button">Back</button>
    <button type="button" class="update-rankings-button" on:click={handleUpdateRankings}>
        Update Rankings
    </button>
    <p class="refresh-date">{refreshDate}</p>

    {#if showPopup}
        <div class="popup-background">
            <span class="background-helper"></span>
            <div class="popup-content">
                <p>
                    Choose your league's scoring settings below.
                    <br/>It might take up to 10 minutes to download everything.
                </p>
                <button class="popup-scoring-button" on:click={() => handleScoringSelect(ScoringType.STANDARD)}>
                    Standard
                </button>
                <button class="popup-scoring-button" on:click={() => handleScoringSelect(ScoringType.HALF)}>
                    Half PPR
                </button>
                <button class="popup-scoring-button" on:click={() => handleScoringSelect(ScoringType.PPR)}>
                    Full PPR
                </button>
                <button class="popup-cancel-button" on:click={() => showPopup = false}>
                    Cancel
                </button>
            </div>
        </div>
    {/if}

    {#if loading}
        <div class="loader">
            <img class="loader-image" src="/img/loader.gif" alt="Loading..." />
        </div>
    {/if}
</div> 