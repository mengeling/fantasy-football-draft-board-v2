<script lang="ts">
    import { tick } from 'svelte';
    import { Team, Position } from '$lib/enums';
    import { defaultPlayer, type Player } from '$lib/types';
    import { fetchApi } from '$lib/api';
    import { POSITION_STATS } from '$lib/constants';
    import { fmtNum, fmtPts } from '$lib/format';

    export let players: Player[] = [];
    export let selectedPlayer: Player = defaultPlayer;
    export let onPlayerDraftChange: (player: Player) => void;
    export let userId: string | undefined;

    let showAvailablePlayers = true;
    let positionFilter: Position = Position.ALL;
    let teamFilter: Team = Team.ALL;
    let playerNameSearch: string | null = null;

    // Position-specific stat columns are only shown once a single position is
    // selected — then every visible row shares them. PTS is already a universal
    // column, so drop the duplicate.
    // PTS, PPG and G are always-shown universal columns, so drop them from the
    // position-specific group to avoid duplicate columns.
    $: statColumns =
        positionFilter === Position.ALL
            ? []
            : POSITION_STATS[positionFilter].filter(
                  (stat) => stat.key !== 'points' && stat.key !== 'games'
              );

    async function toggleDraft(player: Player) {
        if (!player.id) return;

        const method = player.drafted ? 'DELETE' : 'POST';
        try {
            await fetchApi(`/drafted_players/${player.id}`, { method, userId });
        } catch (error) {
            console.error('Failed to update draft status:', error);
            return;
        }

        onPlayerDraftChange({ ...player, drafted: !player.drafted });
        playerNameSearch = null;
    }

    function handleDraftAction() {
        toggleDraft(selectedPlayer);
    }

    const posClass = (position: Position | null): string =>
        position ? `pos-${position.toLowerCase()}` : '';

    async function handleKeydown(event: KeyboardEvent) {
        // Don't hijack typing in the search box / dropdowns.
        const tag = (document.activeElement?.tagName ?? '').toLowerCase();
        if (tag === 'input' || tag === 'select' || tag === 'textarea') return;

        if (event.key === 'ArrowDown' || event.key === 'ArrowUp') {
            if (filteredPlayers.length === 0) return;
            event.preventDefault();
            const current = filteredPlayers.findIndex((p) => p.id === selectedPlayer.id);
            const delta = event.key === 'ArrowDown' ? 1 : -1;
            const next = current < 0 ? 0 : Math.min(filteredPlayers.length - 1, Math.max(0, current + delta));
            selectedPlayer = filteredPlayers[next];
            await tick();
            document
                .querySelector(`[data-player-id="${selectedPlayer.id}"]`)
                ?.scrollIntoView({ block: 'nearest' });
        } else if (event.key === 'Enter' && selectedPlayer.id) {
            event.preventDefault();
            toggleDraft(selectedPlayer);
        }
    }

    $: filteredPlayers = players.filter((player) => {
        const matchesPosition = positionFilter === Position.ALL || player.position === positionFilter;
        const matchesTeam = teamFilter === Team.ALL || player.team === teamFilter;
        const matchesSearch =
            !playerNameSearch || player.name.toLowerCase().includes(playerNameSearch.toLowerCase());
        const matchesAvailability = showAvailablePlayers ? !player.drafted : player.drafted;

        return matchesPosition && matchesTeam && matchesSearch && matchesAvailability;
    });

    $: {
        if (filteredPlayers.length === 0) {
            selectedPlayer = defaultPlayer;
        } else if (!filteredPlayers.includes(selectedPlayer)) {
            selectedPlayer = filteredPlayers[0];
        }
    }
</script>

<style>
    .board-container {
        width: 98%;
        margin: 0 auto;
    }

    .board-toolbar {
        display: flex;
        align-items: center;
        gap: 14px;
        flex-wrap: wrap;
        background: var(--panel);
        border: 1px solid var(--border);
        border-radius: 12px 12px 0 0;
        padding: 10px 14px;
        text-align: left;
    }

    .segmented {
        display: inline-flex;
        border: 1px solid var(--border-strong);
        border-radius: var(--radius);
        overflow: hidden;
    }

    .segmented button {
        border: none;
        border-radius: 0;
        background: var(--surface);
        font-size: 0.78rem;
        padding: 6px 12px;
    }

    .segmented button.active {
        background: var(--accent);
        color: #fff;
    }

    .segmented button.active:hover {
        background: var(--accent-hover);
    }

    .filter {
        display: inline-flex;
        align-items: center;
        gap: 6px;
        font-size: 0.8rem;
        color: var(--text-muted);
    }

    select,
    .player-search {
        font-family: inherit;
        font-size: 0.8rem;
        padding: 5px 9px;
        border: 1px solid var(--border-strong);
        border-radius: var(--radius);
        background: var(--surface);
        color: var(--text);
    }

    .player-search:focus,
    select:focus {
        outline: none;
        border-color: var(--accent);
    }

    .draft-button {
        font-size: 0.82rem;
        padding: 7px 14px;
        border: 1px solid var(--accent);
        background: var(--accent);
        color: #fff;
    }

    .draft-button:hover:not(:disabled) {
        background: var(--accent-hover);
        border-color: var(--accent-hover);
    }

    .draft-button.undraft {
        background: var(--danger);
        border-color: var(--danger);
    }

    .table-wrapper {
        border: 1px solid var(--border);
        border-top: none;
        border-radius: 0 0 12px 12px;
        overflow: hidden;
    }

    .draft-board {
        max-height: calc(100vh - 330px);
        min-height: 240px;
        overflow: auto;
    }

    .draft-board table {
        width: 100%;
        table-layout: fixed;
        font-size: 0.74rem;
    }

    /* RANK narrow, PLAYER bounded; remaining width is split evenly across the
       numeric columns so they spread to fill instead of clustering. */
    .draft-board th:nth-child(1),
    .draft-board td:nth-child(1) {
        width: 3rem;
    }

    .draft-board th:nth-child(2),
    .draft-board td:nth-child(2) {
        width: 15rem;
    }

    .draft-board thead th {
        position: sticky;
        top: 0;
        z-index: 1;
        background: var(--panel-strong);
        color: var(--text);
        font-size: 0.72rem;
        letter-spacing: 0.02em;
        white-space: nowrap;
        padding: 7px 8px;
        border-bottom: 1px solid var(--border-strong);
    }

    .draft-board tbody td {
        padding: 5px 8px;
        white-space: nowrap;
        border: none;
        border-bottom: 1px solid var(--border);
    }

    .draft-board tbody tr:nth-child(odd) td {
        background: var(--row-odd);
    }

    .draft-board tbody tr:nth-child(even) td {
        background: var(--row-even);
    }

    .draft-board tbody tr:hover td {
        background: var(--row-hover);
    }

    .draft-board tbody tr.selected td {
        background: var(--row-selected);
    }

    .draft-board tbody tr {
        cursor: pointer;
    }

    .player-cell {
        overflow: hidden;
        text-overflow: ellipsis;
    }

    .team {
        color: var(--text-muted);
        font-size: 0.92em;
    }

    .hint {
        font-size: 0.72rem;
        color: var(--text-muted);
        margin: 6px 2px 0;
        text-align: left;
    }

    .empty {
        padding: 28px;
        text-align: center;
        color: var(--text-muted);
        font-size: 0.85rem;
    }
</style>

<svelte:window on:keydown={handleKeydown} />

<div class="board-container">
    <div class="board-toolbar">
        <div class="segmented">
            <button
                type="button"
                class:active={showAvailablePlayers}
                on:click={() => (showAvailablePlayers = true)}
            >
                Available
            </button>
            <button
                type="button"
                class:active={!showAvailablePlayers}
                on:click={() => (showAvailablePlayers = false)}
            >
                Drafted
            </button>
        </div>

        <span class="filter">
            Position
            <select bind:value={positionFilter}>
                {#each Object.values(Position) as positionOption}
                    <option value={positionOption}>{positionOption}</option>
                {/each}
            </select>
        </span>

        <span class="filter">
            Team
            <select bind:value={teamFilter}>
                {#each Object.values(Team) as teamOption}
                    <option value={teamOption}>{teamOption}</option>
                {/each}
            </select>
        </span>

        <input
            type="search"
            placeholder="Search player"
            class="player-search"
            bind:value={playerNameSearch}
        />

        <button
            type="button"
            class="draft-button"
            class:undraft={selectedPlayer.drafted}
            on:click={handleDraftAction}
            disabled={!selectedPlayer.id}
        >
            {selectedPlayer.drafted ? 'Undraft selected' : 'Draft selected'}
        </button>
    </div>

    <div class="table-wrapper">
        <div class="draft-board">
            {#if filteredPlayers.length > 0}
                <table>
                    <thead>
                        <tr>
                            <th>RANK</th>
                            <th>PLAYER</th>
                            <th>BYE</th>
                            <th>POS RK</th>
                            <th>BEST</th>
                            <th>WORST</th>
                            <th>AVG</th>
                            <th>STDEV</th>
                            <th>PTS</th>
                            <th>PPG</th>
                            <th>G</th>
                            {#each statColumns as col}
                                <th>{col.label}</th>
                            {/each}
                        </tr>
                    </thead>
                    <tbody>
                        {#each filteredPlayers as player (player.id)}
                            <tr
                                class:selected={selectedPlayer.id === player.id}
                                data-player-id={player.id}
                                on:click={() => (selectedPlayer = player)}
                                on:dblclick={() => toggleDraft(player)}
                                role="button"
                                tabindex="0"
                                title="Double-click to {player.drafted ? 'undraft' : 'draft'}"
                            >
                                <td>{fmtNum(player.rankings.overall)}</td>
                                <td class="player-cell">
                                    <span class="pos-badge {posClass(player.position)}">{player.position}</span
                                    >{player.name} <span class="team">{player.team}</span>
                                </td>
                                <td>{fmtNum(player.bye_week)}</td>
                                <td>{fmtNum(player.rankings.position)}</td>
                                <td>{fmtNum(player.rankings.best)}</td>
                                <td>{fmtNum(player.rankings.worst)}</td>
                                <td>{fmtNum(player.rankings.average)}</td>
                                <td>{fmtNum(player.rankings.standard_deviation)}</td>
                                <td>{fmtPts(player.stats.points)}</td>
                                <td>{fmtNum(player.stats.points_per_game)}</td>
                                <td>{fmtNum(player.stats.games)}</td>
                                {#each statColumns as col}
                                    <td>{fmtNum(player.stats[col.key])}</td>
                                {/each}
                            </tr>
                        {/each}
                    </tbody>
                </table>
            {:else}
                <p class="empty">
                    No {showAvailablePlayers ? 'available' : 'drafted'} players match these filters.
                </p>
            {/if}
        </div>
    </div>

    <p class="hint">
        Click or use ↑ ↓ to preview · double-click or Enter to {showAvailablePlayers
            ? 'draft'
            : 'undraft'}
    </p>
</div>
