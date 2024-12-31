<script>
    import { onMount } from 'svelte';
    import { fetchApi } from '$lib/api';

    let data = [];

    onMount(async () => {
        try {
            // Replace with your actual API endpoint
            data = await fetchApi('/api/your-endpoint');
        } catch (error) {
            console.error('Failed to fetch data:', error);
        }
    });
</script>

<main>
    <h1>Welcome to Your New Frontend</h1>
</main>
