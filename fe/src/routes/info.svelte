<script lang="ts">
    export let toggleShowGameInfo: () => void;
</script>

<div class="info-section">
    <p>
        There are two players in the game: a bot player and you, most likely a human player.
        Starting player is selected randomly for each game.
    </p>
    <p>
        Players alternate turns placing an X or O marker on an empty cell of the game board. The
        winner is the first player to have k markers consecutively in a row either vertically,
        horizontally or diagonally.
    </p>
    <p>
        Some of the available k-in-a-row games have special names: the smallest game, 3x3
        3-in-a-row, is the traditional tic-tac-toe, and the largest, 15x15 5-in-a-row, is the
        Freestyle gomoku (no restrictions on opening moves).
    </p>
    <p>
        Bot player uses a variation of the Minimax algorithm with alpha-beta pruning for its game
        playing strategy.
    </p>
    <button id="info-close" on:click={toggleShowGameInfo}>Close</button>
</div>

<style>
    .info-section {
        background-color: var(--default-white);
        display: flex;
        flex-direction: column;
        position: relative;
        width: 50vw;
        z-index: 2;
    }
    .info-section button {
        margin: 1em 0;
        padding: 0.5em 1em;
        border: none;
        border-radius: 0.5em;
        font-size: 1.5rem;
        font-weight: bold;
        cursor: pointer;
    }
    @media (prefers-color-scheme: dark) {
        .info-section {
            background-color: var(--default-black);
        }
    }
    @media screen and (max-width: 800px) {
        .info-section {
            width: 60vw;
        }
    }
</style>
