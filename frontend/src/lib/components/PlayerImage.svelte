<script lang="ts">
    export let id: number;
    export let name: string;

    $: imgUrl = `https://images.fantasypros.com/images/players/nfl/${id}/headshot/250x250.png`;
    const fallbackImgUrl = 'https://images.fantasypros.com/images/photo_missing_square.jpg';
</script>

<style>
    .player-pic {
        width: 118px;
        height: 118px;
        object-fit: cover;
        border-radius: 14px;
        background: var(--panel);
        flex-shrink: 0;
        display: block;
    }
</style>

<img
    class="player-pic"
    id={id.toString()}
    src={imgUrl}
    on:error={(e) => ((e.target as HTMLImageElement).src = fallbackImgUrl)}
    alt={name}
/>
