<script lang="ts">
    export let draftedCount: number;
    export let onConfirm: () => void;
    export let onCancel: () => void;
</script>

<style>
    .login-scoring-button,
    .login-cancel-button {
        font-size: 0.85em;
        margin: 1%;
        padding: 1%;
    }
</style>

<div class="login-background">
    <span class="login-helper"></span>
    <div class="login-content">
        <p>
            Clear all {draftedCount} drafted {draftedCount === 1 ? 'player' : 'players'} from your board?<br/>
            This can't be undone.
        </p>
        <button class="login-scoring-button" on:click={onConfirm}>
            Reset Board
        </button>
        <button class="login-cancel-button" on:click={onCancel}>
            Cancel
        </button>
    </div>
</div>
