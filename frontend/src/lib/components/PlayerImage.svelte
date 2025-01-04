<script lang="ts">
    export let id: string;
    export let imgUrl: string;
    export let name: string;
    
    const fallbackImage = 'https://images.fantasypros.com/images/photo_missing_square.jpg';
</script>

<div class="player-pic-container">
    <img 
        class="player-pic" 
        {id}
        src={imgUrl} 
        on:error={(e) => (e.target as HTMLImageElement).src = fallbackImage}
        alt={name}
    >
</div>
