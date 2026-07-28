<script lang="ts">
    export let draftedCount: number;
    export let onConfirm: () => void;
    export let onCancel: () => void;
</script>

<div class="login-background">
    <span class="login-helper"></span>
    <div class="login-content">
        <p class="modal-title">Reset draft board?</p>
        <p class="modal-subtitle">
            This clears {draftedCount} drafted {draftedCount === 1 ? 'player' : 'players'} from your
            board and can't be undone.
        </p>
        <div class="modal-actions">
            <button class="btn-danger" on:click={onConfirm}>Reset board</button>
            <button class="btn-ghost" on:click={onCancel}>Cancel</button>
        </div>
    </div>
</div>
