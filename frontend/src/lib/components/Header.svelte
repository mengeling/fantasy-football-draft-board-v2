<script lang="ts">
    import { onMount } from 'svelte';
    import { fetchApi } from '$lib/api';

    export let username = '';
    export let onLogout: () => void;
    export let onUpdateScoring: () => void;
    export let onResetBoard: () => void;
    export let draftedCount = 0;
    export let loading = false;

    let refreshDate = '';
    let menuOpen = false;
    let menuEl: HTMLDivElement;

    $: initials = username
        .split(' ')
        .filter(Boolean)
        .slice(0, 2)
        .map((part) => part[0]?.toUpperCase() ?? '')
        .join('');

    function toggleMenu() {
        menuOpen = !menuOpen;
    }

    function closeMenu() {
        menuOpen = false;
    }

    function onWindowClick(event: MouseEvent) {
        if (menuOpen && menuEl && !menuEl.contains(event.target as Node)) {
            closeMenu();
        }
    }

    function onKeydown(event: KeyboardEvent) {
        if (event.key === 'Escape') closeMenu();
    }

    function runAndClose(action: () => void) {
        action();
        closeMenu();
    }

    async function fetchLastUpdate() {
        try {
            const data = await fetchApi('/fantasy-data/last-update');
            refreshDate = new Date(data.last_update).toLocaleString(undefined, {
                year: 'numeric',
                month: 'numeric',
                day: 'numeric'
            });
        } catch (error) {
            console.error('Error fetching last update time:', error);
        }
    }

    onMount(() => {
        fetchLastUpdate();
    });
</script>

<svelte:window on:click={onWindowClick} on:keydown={onKeydown} />

<style>
    .header-bar {
        width: 98%;
        margin: 0.6rem auto 1rem;
        display: flex;
        justify-content: space-between;
        align-items: center;
        gap: 12px;
    }

    .brand {
        display: flex;
        align-items: center;
        gap: 10px;
    }

    .brand-mark {
        display: block;
        flex-shrink: 0;
    }

    .brand-word {
        font-family: var(--font-brand);
        font-size: 1.25rem;
        font-weight: 700;
        color: var(--text);
        letter-spacing: -0.01em;
    }

    .header-right {
        display: flex;
        align-items: center;
        gap: 16px;
    }

    .refresh-date {
        font-size: 0.78rem;
        color: var(--text-muted);
        margin: 0;
        white-space: nowrap;
    }

    .user-menu {
        position: relative;
    }

    .user-menu-button {
        display: inline-flex;
        align-items: center;
        gap: 8px;
        font-size: 0.85rem;
        padding: 6px 12px;
    }

    .avatar {
        width: 22px;
        height: 22px;
        border-radius: 50%;
        background: var(--accent-soft);
        color: var(--accent-hover);
        display: inline-flex;
        align-items: center;
        justify-content: center;
        font-size: 0.68rem;
        font-weight: 600;
    }

    .caret {
        font-size: 0.6rem;
        color: var(--text-muted);
    }

    .menu {
        position: absolute;
        right: 0;
        top: calc(100% + 6px);
        min-width: 210px;
        background: var(--surface);
        border: 1px solid var(--border);
        border-radius: 12px;
        box-shadow: 0 12px 30px rgba(20, 30, 45, 0.16);
        padding: 6px;
        z-index: 40;
        text-align: left;
    }

    .menu-item {
        display: flex;
        align-items: center;
        gap: 10px;
        width: 100%;
        border: none;
        background: transparent;
        border-radius: var(--radius);
        padding: 8px 10px;
        font-size: 0.85rem;
        color: var(--text);
        text-align: left;
    }

    .menu-item:hover:not(:disabled) {
        background: var(--accent-soft);
        border-color: transparent;
    }

    .menu-item.danger {
        color: var(--danger);
    }

    .menu-item.danger:hover:not(:disabled) {
        background: var(--danger-soft);
    }

    .menu-divider {
        height: 1px;
        background: var(--border);
        margin: 5px 8px;
    }
</style>

<div class="header-bar">
    <div class="brand">
        <svg class="brand-mark" viewBox="0 0 32 32" width="30" height="30" role="img" aria-label="Draft Board logo">
            <rect width="32" height="32" rx="9" fill="var(--accent)" />
            <rect x="7" y="8" width="5" height="5" rx="1.5" fill="#ffffff" />
            <rect x="14" y="9.5" width="11" height="2.4" rx="1.2" fill="#ffffff" opacity="0.95" />
            <rect x="7" y="15" width="5" height="5" rx="1.5" fill="#ffffff" opacity="0.8" />
            <rect x="14" y="16.5" width="11" height="2.4" rx="1.2" fill="#ffffff" opacity="0.75" />
            <rect x="7" y="22" width="5" height="5" rx="1.5" fill="#ffffff" opacity="0.55" />
            <rect x="14" y="23.5" width="11" height="2.4" rx="1.2" fill="#ffffff" opacity="0.5" />
        </svg>
        <span class="brand-word">Draft Board</span>
    </div>

    <div class="header-right">
        <p class="refresh-date">Rankings updated {refreshDate}</p>

        <div class="user-menu" bind:this={menuEl}>
            <button
                class="user-menu-button"
                on:click|stopPropagation={toggleMenu}
                aria-haspopup="true"
                aria-expanded={menuOpen}
            >
                <span class="avatar">{initials}</span>
                <span>{username}</span>
                <span class="caret" aria-hidden="true">▼</span>
            </button>

            {#if menuOpen}
                <div class="menu" role="menu">
                    <button
                        class="menu-item"
                        role="menuitem"
                        on:click={() => runAndClose(onUpdateScoring)}
                        disabled={loading}
                    >
                        Scoring settings
                    </button>
                    <button
                        class="menu-item danger"
                        role="menuitem"
                        on:click={() => runAndClose(onResetBoard)}
                        disabled={loading || draftedCount === 0}
                    >
                        Reset draft board{draftedCount > 0 ? ` (${draftedCount})` : ''}
                    </button>
                    <div class="menu-divider"></div>
                    <button
                        class="menu-item"
                        role="menuitem"
                        on:click={() => runAndClose(onLogout)}
                    >
                        Log out
                    </button>
                </div>
            {/if}
        </div>
    </div>
</div>
