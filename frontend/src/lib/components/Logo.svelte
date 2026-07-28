<script lang="ts">
    export let size = 30;
</script>

<style>
    .logo {
        display: block;
        flex-shrink: 0;
    }
</style>

<svg
    class="logo"
    viewBox="0 0 32 32"
    width={size}
    height={size}
    role="img"
    aria-label="Draft Board logo"
>
    <rect width="32" height="32" rx="9" fill="var(--accent)" />
    <rect x="7" y="8" width="5" height="5" rx="1.5" fill="#ffffff" />
    <rect x="14" y="9.5" width="11" height="2.4" rx="1.2" fill="#ffffff" opacity="0.95" />
    <rect x="7" y="15" width="5" height="5" rx="1.5" fill="#ffffff" opacity="0.8" />
    <rect x="14" y="16.5" width="11" height="2.4" rx="1.2" fill="#ffffff" opacity="0.75" />
    <rect x="7" y="22" width="5" height="5" rx="1.5" fill="#ffffff" opacity="0.55" />
    <rect x="14" y="23.5" width="11" height="2.4" rx="1.2" fill="#ffffff" opacity="0.5" />
</svg>
