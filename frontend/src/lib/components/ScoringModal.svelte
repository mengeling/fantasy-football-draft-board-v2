<script lang="ts">
    import { ScoringType } from '$lib/types';
    
    export let onSelect: (scoring: ScoringType) => void;
    export let onCancel: () => void;

    function handleSelect(scoring: ScoringType) {
        onSelect(scoring);
    }

    function handleCancel() {
        onCancel();
    }
</script>

<div class="download-background">
    <span class="download-helper"></span>
    <div class="download-content">
        <p>
            No draft boards exist under that name.<br/>
            Choose a scoring option to create a board.
        </p>
        <button class="login-scoring-button" on:click={() => handleSelect(ScoringType.STANDARD)}>
            Standard
        </button>
        <button class="login-scoring-button" on:click={() => handleSelect(ScoringType.HALF)}>
            Half PPR
        </button>
        <button class="login-scoring-button" on:click={() => handleSelect(ScoringType.PPR)}>
            Full PPR
        </button>
        <button class="login-cancel-button" on:click={handleCancel}>
            Cancel
        </button>
    </div>
</div> 