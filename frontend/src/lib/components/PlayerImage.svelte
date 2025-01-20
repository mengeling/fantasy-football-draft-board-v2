<script lang="ts">
    export let id: number;
    export let name: string;

    $: imgUrl = `https://images.fantasypros.com/images/players/nfl/${id}/headshot/250x250.png`;
    const fallbackImgUrl = 'https://images.fantasypros.com/images/photo_missing_square.jpg';
</script>

<style>
    .player-pic-container {
        height: 160px;
        width: 170px;
        margin: 0;
        float: left;
    }

    .player-pic {
        width: 160px;
    }
</style>

<div class="player-pic-container">
    <img 
        class="player-pic" 
        id={id.toString()}
        src={imgUrl} 
        on:error={(e) => (e.target as HTMLImageElement).src = fallbackImgUrl}
        alt={name}
    >
</div>
