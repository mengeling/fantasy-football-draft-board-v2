<script lang="ts">
    import { ScoringSettings } from '$lib/enums';
    import { fetchApi } from '$lib/api';
    import ScoringModal from './ScoringModal.svelte';
    
    export let onLogin: (username: string, userData: any) => void;

    let username = '';
    let errorMessage = '';
    let loading = false;
    let currentView: 'login' | 'scoring' = 'login';

    async function handleLogin() {        
        loading = true;
        errorMessage = '';
        
        try {
            try {
                const userData = await fetchApi(`/users/${username}`);
                onLogin(username, userData);
            } catch (e) {
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
            const userData = await fetchApi('/users', {
                method: 'POST',
                body: JSON.stringify({
                    username,
                    scoring_settings: scoring,
                })
            });
            
            onLogin(username, userData);
        } catch (error) {
            console.error('Failed to create user:', error);
            errorMessage = 'Failed to connect to server';
        } finally {
            loading = false;
        }
    }
</script>

{#if currentView === 'login'}
    <div class="login-background">
        <span class="login-helper"></span>
        <div class="login-content">
            <form on:submit|preventDefault={handleLogin}>
                <p>Enter Your Full Name</p>
                <div class="username-div">
                    <input 
                        type="search" 
                        bind:value={username}
                        placeholder="First and Last Name" 
                        class="username-input"
                        disabled={loading}
                    >
                </div>
                {#if errorMessage}
                    <p class="error">{errorMessage}</p>
                {/if}
                <button 
                    type="submit" 
                    class="login-button" 
                    disabled={loading}
                >
                    {loading ? 'Loading...' : 'View My Draft Board'}
                </button>
            </form>
        </div>
    </div>
{:else}
    <ScoringModal 
        onSelect={createUserWithScoring}
        onCancel={() => currentView = 'login'}
    />
{/if}

<style>
    .error {
        color: red;
        font-size: 0.9em;
        margin-top: 0.5em;
    }
</style>
