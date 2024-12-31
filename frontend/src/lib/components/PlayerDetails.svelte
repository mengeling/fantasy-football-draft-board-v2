<script lang="ts">
    export let playerData = {
        id: '',
        name: '',
        team: '',
        position: '',
        height: '',
        age: '',
        weight: '',
        college: '',
        img_url: '',
        rankings: '',
        stats: ''
    };
</script>

<div class="player-details">
    <div class="player-pic-container">
        <img 
            class="player-pic" 
            id={playerData.id} 
            src={playerData.img_url} 
            on:error={(e) => (e.target as HTMLImageElement).src = 'https://images.fantasypros.com/images/photo_missing_square.jpg'}
            alt={playerData.name}
        >
    </div>
    <div class="player-bio-container">
        <h3 class="player-name">{playerData.name}</h3>
        <h5 class="player-team-pos">{playerData.team} {playerData.position}</h5>
        <div class="player-bio-sub">
            <p class="player-detail-desc">Height</p>
            <p class="player-detail-height">{playerData.height}</p>
            <p class="player-detail-desc">Age</p>
            <p class="player-detail-age">{playerData.age}</p>
        </div>
        <div class="player-bio-sub">
            <p class="player-detail-desc">Weight</p>
            <p class="player-detail-weight">{playerData.weight}</p>
            <p class="player-detail-desc">College</p>
            <p class="player-detail-college">{playerData.college}</p>
        </div>
    </div>
    <div class="draft-undraft-container">
        <button type="button" id="draft-undraft-button" class="0">
            Draft Selected Player
        </button>
    </div>
    <div class="player-tables">
        <div class="player-table">
            <h4 class="rank-header">Rankings</h4>
            <div class="rank-table">
                {@html playerData.rankings}
            </div>
        </div>
        <div class="player-table">
            <h4 class="stats-header">Previous Stats</h4>
            <div class="stats-table">
                {@html playerData.stats}
            </div>
        </div>
    </div>
</div> 