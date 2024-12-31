<script lang="ts">
    import { createEventDispatcher } from 'svelte';
    const dispatch = createEventDispatcher();

    function handleSelect(scoring: 'standard' | 'half' | 'ppr') {
        dispatch('select', { scoring });
    }

    function handleCancel() {
        dispatch('cancel');
    }
</script>

<div class="download-background">
    <span class="download-helper"></span>
    <div class="download-content">
        <p>
            No draft boards exist under that name.<br/>
            Choose a scoring option to create a board.
        </p>
        <button class="login-scoring-button" on:click={() => handleSelect('standard')}>
            Standard
        </button>
        <button class="login-scoring-button" on:click={() => handleSelect('half')}>
            Half PPR
        </button>
        <button class="login-scoring-button" on:click={() => handleSelect('ppr')}>
            Full PPR
        </button>
        <button class="login-cancel-button" on:click={handleCancel}>
            Cancel
        </button>
    </div>
</div> 