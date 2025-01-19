<script lang="ts">
    export let id: string;
    export let name: string;

    $: imgUrl = `https://images.fantasypros.com/images/players/nfl/${id}/headshot/250x250.png`;
    const fallbackImgUrl = 'https://images.fantasypros.com/images/photo_missing_square.jpg';
</script>

<div class="player-pic-container">
    <img 
        class="player-pic" 
        {id}
        src={imgUrl} 
        on:error={(e) => (e.target as HTMLImageElement).src = fallbackImgUrl}
        alt={name}
    >
</div>
