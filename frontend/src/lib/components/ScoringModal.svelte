<script lang="ts">
    import { ScoringSettings } from '$lib/enums';

    export let onSelect: (scoring: ScoringSettings) => void;
    export let onCancel: () => void;
    export let prompt = 'No board exists under that name yet — pick a format to create one.';

    function handleSelect(scoring: ScoringSettings) {
        onSelect(scoring);
    }

    function handleCancel() {
        onCancel();
    }
</script>

<style>
    .cancel-row {
        margin-top: 8px;
    }
</style>

<div class="login-background">
    <span class="login-helper"></span>
    <div class="login-content">
        <p class="modal-title">Choose a scoring format</p>
        <p class="modal-subtitle">{prompt}</p>
        <div class="modal-actions">
            <button on:click={() => handleSelect(ScoringSettings.Standard)}>Standard</button>
            <button on:click={() => handleSelect(ScoringSettings.Half)}>Half PPR</button>
            <button on:click={() => handleSelect(ScoringSettings.PPR)}>Full PPR</button>
        </div>
        <div class="modal-actions cancel-row">
            <button class="btn-ghost" on:click={handleCancel}>Cancel</button>
        </div>
    </div>
</div>
