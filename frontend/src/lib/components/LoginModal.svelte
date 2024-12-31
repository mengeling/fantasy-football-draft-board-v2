<script lang="ts">
    import { createEventDispatcher } from 'svelte';
    const dispatch = createEventDispatcher();

    let username = '';

    function handleLogin() {
        if (username.trim()) {
            dispatch('login', { username });
        }
    }
</script>

<div class="login-background">
    <span class="login-helper"></span>
    <div class="login-content">
        <p>Enter Your Full Name</p>
        <div class="username-div">
            <input 
                type="search" 
                bind:value={username}
                placeholder="First and Last Name" 
                class="username-input" 
                autofocus
            >
        </div>
        <button type="button" class="login-button" on:click={handleLogin}>
            View My Draft Board
        </button>
    </div>
</div> 