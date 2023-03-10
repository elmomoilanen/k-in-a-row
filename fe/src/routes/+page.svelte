<script lang="ts">
    import { onMount } from "svelte";
    import { error } from "@sveltejs/kit";
    import type { Game, MaybeGame, MaybeGameLevel } from "$lib/games";
    import { getRandomPlayer } from "$lib/players";
    import Board from "./board/+page.svelte";
    import Spinner from "./spinner.svelte";
    import Dropdowns from "./dropdowns.svelte";
    import { PUBLIC_API_URL } from "$env/static/public";

    let currentGameType: MaybeGame = undefined;
    let previousGameType: MaybeGame = undefined;
    let currentGameLevel: MaybeGameLevel = undefined;
    let previousGameLevel: MaybeGameLevel = undefined;
    let startGame = false;
    let backendConnected = false;
    let showGameEndOptions = false;

    function setGameType(newGame: Game) {
        currentGameType = newGame;
    }

    function setGameLevel(newGameLevel: MaybeGameLevel) {
        currentGameLevel = newGameLevel;
    }

    function endGame() {
        previousGameType = currentGameType;
        previousGameLevel = currentGameLevel;
        currentGameType = undefined;
        currentGameLevel = undefined;
        showGameEndOptions = true;
        toggleStartGame();
    }

    function resetGame() {
        currentGameType = previousGameType;
        currentGameLevel = previousGameLevel;
        previousGameType = undefined;
        previousGameLevel = undefined;
        showGameEndOptions = false;
        toggleStartGame();
    }

    function backToStart() {
        currentGameType = undefined;
        currentGameLevel = undefined;
        previousGameType = undefined;
        previousGameLevel = undefined;
        showGameEndOptions = false;
    }

    function toggleStartGame() {
        startGame = startGame ? false : true;
    }

    onMount(async () => {
        const url = `${PUBLIC_API_URL}/api/hello`;
        const resp = await fetch(url);
        if (!resp.ok) {
            throw error(500, { message: `Did not receive OK response from ${url}.` });
        }
        backendConnected = true;
    });
</script>

{#if currentGameType && currentGameLevel && startGame && backendConnected}
    <Board
        gameType={currentGameType}
        gameLevel={currentGameLevel}
        currentPlayer={getRandomPlayer()}
        endGameFn={endGame}
    />
{:else if currentGameType && currentGameLevel && startGame}
    <Spinner />
{:else if showGameEndOptions}
    <div class="open-page" id="game-end-view">
        <h1>Game over!</h1>
        <button on:click={resetGame}>Play again</button>
        <button on:click={backToStart}>Back to start</button>
    </div>
{:else}
    <div class="open-page" id="new-game-view">
        <h1>Let's play k-in-a-row!</h1>
        <Dropdowns
            {setGameType}
            {setGameLevel}
            {toggleStartGame}
            selectedGameType={currentGameType}
            selectedGameLevel={currentGameLevel}
        />
    </div>
{/if}

<style>
    :global(:root) {
        --cell-size: 7em;
        --mark-size: calc(var(--cell-size) * 0.9);
        --cell-size-mobile-small-board: 4.5em;
        --mark-size-mobile-small-board: calc(var(--cell-size-mobile-small-board) * 0.9);
        --cell-size-mobile-mid-board: 3.75em;
        --mark-size-mobile-mid-board: calc(var(--cell-size-mobile-mid-board) * 0.9);
        --cell-size-large-board: 4.5em;
        --mark-size-large-board: calc(var(--cell-size-large-board) * 0.9);
        --cell-size-mobile-large-board: 3.5em;
        --mark-size-mobile-large-board: calc(var(--cell-size-mobile-large-board) * 0.9);
        --default-black: #202020;
        --default-white: #fafafa;
        --default-light-green: #95e395;
        --default-light-green-hover: #79bb79;
        --default-gray: grey;
        --default-light-gray: #d7d7d7;
        --default-light-gray-hover: #bcbcbc;
        --default-mid-gray-hover: rgb(139, 138, 138);
    }
    :global(body) {
        margin: 0;
        background-color: var(--default-white);
        transition: background-color 0.3s;
    }
    @media (prefers-color-scheme: dark) {
        :global(body) {
            background-color: var(--default-black);
        }
    }
    .open-page {
        position: fixed;
        display: flex;
        flex-direction: column;
        align-items: center;
        top: 0;
        bottom: 0;
        left: 0;
        right: 0;
    }
    .open-page h1 {
        margin: 2em 0 1.5em;
    }
    .open-page button {
        margin: 0.5em;
        padding: 0.5em 1em;
        border: none;
        border-radius: 0.5em;
        background-color: var(--default-light-green);
        color: var(--default-black);
        font-size: 1.5em;
        font-weight: bold;
        cursor: pointer;
        transition: background-color 0.3s, color 0.3s;
    }
    .open-page button:hover {
        background-color: var(--default-light-green-hover);
    }
    @media screen and (max-width: 450px) {
        .open-page h1 {
            font-size: 1.75em;
        }
    }
</style>
