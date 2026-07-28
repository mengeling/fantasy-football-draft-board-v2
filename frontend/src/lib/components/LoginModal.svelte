<script lang="ts">
    import { ScoringSettings } from '$lib/enums';
    import { fetchApi } from '$lib/api';
    import type { User } from '$lib/types';
    import ScoringModal from './ScoringModal.svelte';
    
    export let onLogin: (user: User) => void;

    let username = '';
    let errorMessage = '';
    let loading = false;
    let currentView: 'login' | 'scoring' = 'login';

    async function handleLogin() {        
        loading = true;
        errorMessage = '';
        
        try {
            try {
                const user = await fetchApi(`/users/${encodeURIComponent(username.trim())}`);
                onLogin(user);
            } catch {
                currentView = 'scoring';
            }
        } catch (error) {
            console.error('Login error:', error);
            errorMessage = 'Failed to connect to server';
        } finally {
            loading = false;
        }
    }

    async function createUserWithScoring(scoring: ScoringSettings) {
        loading = true;
        try {
            const user = await fetchApi('/users', {
                method: 'POST',
                body: JSON.stringify({
                    username,
                    scoring_settings: scoring,
                })
            });
            onLogin(user);
        } catch (error) {
            console.error('Failed to create user:', error);
            errorMessage = 'Failed to connect to server';
        } finally {
            loading = false;
        }
    }
</script>

<style>
    .username-div {
        margin-top: 18px;
    }

    .username-input {
        width: 100%;
        box-sizing: border-box;
        font-size: 0.9rem;
        padding: 9px 12px;
        border: 1px solid var(--border-strong);
        border-radius: var(--radius);

        &:focus {
            outline: none;
            border-color: var(--accent);
        }
    }

    .error {
        color: var(--danger);
        font-size: 0.85rem;
        margin-top: 8px;
    }
</style>

{#if currentView === 'login'}
    <div class="login-background">
        <span class="login-helper"></span>
        <div class="login-content">
            <form on:submit|preventDefault={handleLogin}>
                <p class="modal-title">Enter your full name</p>
                <p class="modal-subtitle">This will open your board or create a new one.</p>
                <div class="username-div">
                    <input
                        type="search"
                        bind:value={username}
                        placeholder="First and last name"
                        class="username-input"
                        disabled={loading}
                    >
                </div>
                {#if errorMessage}
                    <p class="error">{errorMessage}</p>
                {/if}
                <div class="modal-actions">
                    <button type="submit" class="btn-primary" disabled={loading}>
                        {loading ? 'Loading…' : 'View my draft board'}
                    </button>
                </div>
            </form>
        </div>
    </div>
{:else}
    <ScoringModal 
        onSelect={createUserWithScoring}
        onCancel={() => currentView = 'login'}
    />
{/if}
