<script lang="ts">
    import { ScoringSettings } from '$lib/types';
    import ScoringModal from './ScoringModal.svelte';
    export let onLogin: (username: string, userData: any) => void;

    let username = '';
    let errorMessage = '';
    let loading = false;
    let showScoringOptions = false;

    async function handleLogin() {
        if (!username.trim()) return;
        
        loading = true;
        errorMessage = '';
        
        try {
            // First check if user exists
            const response = await fetch(`/api/user/${username}`);
            
            if (response.ok) {
                // User exists, use their existing data
                const userData = await response.json();
                console.log('Existing user found:', userData);
                onLogin(username, userData);
            } else if (response.status === 404) {
                // User doesn't exist, show scoring options
                showScoringOptions = true;
            } else {
                errorMessage = 'Something went wrong';
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
            const createResponse = await fetch('/api/user', {
                method: 'POST',
                headers: {
                    'Content-Type': 'application/json'
                },
                body: JSON.stringify({
                    username,
                    scoring_settings: { type: scoring }
                })
            });

            if (createResponse.ok) {
                const newUser = await createResponse.json();
                console.log('New user created:', newUser);
                onLogin(username, newUser);
            } else {
                errorMessage = 'Failed to create user';
                showScoringOptions = false;
            }
        } catch (error) {
            console.error('Create user error:', error);
            errorMessage = 'Failed to connect to server';
            showScoringOptions = false;
        } finally {
            loading = false;
        }
    }
</script>

<div class="login-background" class:hidden={showScoringOptions}>
    <span class="login-helper"></span>
    <div class="login-content">
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
            type="button" 
            class="login-button" 
            on:click={handleLogin}
            disabled={loading}
        >
            {loading ? 'Loading...' : 'View My Draft Board'}
        </button>
    </div>
</div>

{#if showScoringOptions}
    <ScoringModal 
        onSelect={createUserWithScoring}
        onCancel={() => showScoringOptions = false}
    />
{/if}

<style>
    .error {
        color: red;
        font-size: 0.9em;
        margin-top: 0.5em;
    }

    .hidden {
        display: none;
    }
</style>
