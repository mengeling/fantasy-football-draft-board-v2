<script lang="ts">
    import { ScoringSettings } from '$lib/enums';
    
    export let onSelect: (scoring: ScoringSettings) => void;
    export let onCancel: () => void;

    function handleSelect(scoring: ScoringSettings) {
        onSelect(scoring);
    }

    function handleCancel() {
        onCancel();
    }
</script>

<div class="login-background">
    <span class="login-helper"></span>
    <div class="login-content">
        <p>
            No draft boards exist under that name.<br/>
            Choose a scoring option to create a board.
        </p>
        <button class="login-scoring-button" on:click={() => handleSelect(ScoringSettings.Standard)}>
            Standard
        </button>
        <button class="login-scoring-button" on:click={() => handleSelect(ScoringSettings.Half)}>
            Half PPR
        </button>
        <button class="login-scoring-button" on:click={() => handleSelect(ScoringSettings.PPR)}>
            Full PPR
        </button>
        <button class="login-cancel-button" on:click={handleCancel}>
            Cancel
        </button>
    </div>
</div> 