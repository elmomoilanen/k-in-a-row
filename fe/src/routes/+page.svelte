<script lang="ts">
    import { GameType } from "$lib/games";
    import { GameLevel } from "$lib/levels";
    import { Player } from "$lib/players";
    import Board from "./board.svelte";

    let currentGameType = GameType.None;
    let currentGameLevel = GameLevel.None;
    let previousGameType = GameType.None;
    let previousGameLevel = GameLevel.None;
    let showGameEndOptions = false;

    function setGameType(newGameType: GameType) {
        currentGameType = newGameType;
    }

    function setGameLevel(newGameLevel: GameLevel) {
        currentGameLevel = newGameLevel;
    }

    function endGame() {
        previousGameType = currentGameType;
        previousGameLevel = currentGameLevel;
        currentGameType = GameType.None;
        currentGameLevel = GameLevel.None;
        showGameEndOptions = true;
    }

    function resetGame() {
        currentGameType = previousGameType;
        currentGameLevel = previousGameLevel;
        previousGameType = GameType.None;
        previousGameLevel = GameLevel.None;
        showGameEndOptions = false;
    }

    function backToStart() {
        currentGameType = GameType.None;
        currentGameLevel = GameLevel.None;
        previousGameType = GameType.None;
        previousGameLevel = GameLevel.None;
        showGameEndOptions = false;
    }

    function getRandomPlayer() {
        return Math.random() > 0.5 ? Player.P1 : Player.Bot;
    }
</script>

{#if currentGameType !== GameType.None && currentGameLevel !== GameLevel.None}
    <Board
        size={currentGameType}
        gameLevel={currentGameLevel}
        currentPlayer={getRandomPlayer()}
        endGameFn={endGame}
    />
{:else if currentGameType !== GameType.None}
    <div class="open-page" id="select-level">
        <h3>Select level</h3>
        <button on:click={() => setGameLevel(GameLevel.Easy)}>Easy</button>
        <button on:click={() => setGameLevel(GameLevel.Normal)}>Normal</button>
    </div>
{:else if showGameEndOptions}
    <div class="open-page" id="reset-game">
        <h3>Choose an action</h3>
        <button on:click={resetGame}>Play again</button>
        <button on:click={backToStart}>Back to start</button>
    </div>
{:else}
    <div class="open-page" id="select-game">
        <h1>Welcome to play k-in-a-row!</h1>
        <h3>Start a new game</h3>
        <button on:click={() => setGameType(GameType.X33)}>3x3 3-in-a-row</button>
        <button on:click={() => setGameType(GameType.X44)}>4x4 4-in-a-row</button>
        <button on:click={() => setGameType(GameType.X55)}>5x5 4-in-a-row</button>
        <button on:click={() => setGameType(GameType.X66)}>6x6 5-in-a-row</button>
        <button on:click={() => setGameType(GameType.X77)}>7x7 5-in-a-row</button>
    </div>
{/if}

<style>
    :global(:root) {
        --cell-size: 7em;
        --mark-size: calc(var(--cell-size) * 0.9);
        --cell-size-small: 4.5em;
        --mark-size-small: calc(var(--cell-size-small) * 0.9);
    }
    .open-page {
        position: fixed;
        display: flex;
        flex-direction: column;
        justify-content: center;
        align-items: center;
        top: 0;
        bottom: 0;
        left: 0;
        right: 0;
        background-color: white;
    }
    .open-page button {
        padding: 0.2em 0.5em;
        margin-bottom: 0.8em;
        border-radius: 100px;
        cursor: pointer;
        font-size: 2rem;
        background: linear-gradient(145deg, #f0f0f0, #9cd563);
    }
    .open-page button:hover {
        color: white;
        border-color: white;
    }
    @media screen and (max-width: 450px) {
        .open-page h1,
        .open-page button {
            font-size: 1.75em;
        }
    }
</style>
