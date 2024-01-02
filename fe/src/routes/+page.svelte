<script lang="ts">
    import { onMount } from "svelte";
    import type { Game, MaybeGame, MaybeGameLevel } from "$lib/games";
    import { getRandomPlayer, Player } from "$lib/players";
    import type { MaybePlayer } from "$lib/players";
    import { getPhrase } from "$lib/phrases";
    import Board from "./board/+page.svelte";
    import Spinner from "./spinner.svelte";
    import Dropdowns from "./dropdowns.svelte";
    import Errors from "./errors.svelte";
    import Info from "./info.svelte";
    import { PUBLIC_API_URL } from "$env/static/public";

    const SHOW_PHRASES_LIMIT = 0.5;

    let currentGameType: MaybeGame = undefined;
    let previousGameType: MaybeGame = undefined;
    let currentGameLevel: MaybeGameLevel = undefined;
    let previousGameLevel: MaybeGameLevel = undefined;
    let currentWinner: MaybePlayer = undefined;
    let startGame = false;
    let backendConnected = false;
    let showGameEndOptions = false;
    let showGameInfo = false;
    let apiErrorOccurred = false;

    function setGameType(newGame: Game): void {
        currentGameType = newGame;
    }

    function setGameLevel(newGameLevel: MaybeGameLevel): void {
        currentGameLevel = newGameLevel;
    }

    function endGame(gameWinner: Player): void {
        previousGameType = currentGameType;
        previousGameLevel = currentGameLevel;
        currentGameType = undefined;
        currentGameLevel = undefined;
        currentWinner = gameWinner;
        showGameEndOptions = true;
        toggleStartGame();
    }

    function resetGame(): void {
        currentGameType = previousGameType;
        currentGameLevel = previousGameLevel;
        previousGameType = undefined;
        previousGameLevel = undefined;
        currentWinner = undefined;
        showGameEndOptions = false;
        toggleStartGame();
    }

    function backToStart(): void {
        currentGameType = undefined;
        currentGameLevel = undefined;
        previousGameType = undefined;
        previousGameLevel = undefined;
        currentWinner = undefined;
        showGameEndOptions = false;
    }

    function toggleStartGame(): void {
        startGame = !startGame;
    }

    function toggleShowGameInfo(): void {
        showGameInfo = !showGameInfo;
    }

    function isCellsToWinWithinLimits(gameType: MaybeGame): boolean {
        return (
            !!gameType &&
            gameType.cellsToWin >= gameType.cellsToWinMin &&
            gameType.cellsToWin <= gameType.cellsToWinMax
        );
    }

    onMount(async (): Promise<void> => {
        const url = `${PUBLIC_API_URL}/api/hello`;
        let resp;
        try {
            resp = await fetch(url);
        } catch (error) {
            console.error(error);
            apiErrorOccurred = true;
            return;
        }
        if (!resp.ok) {
            console.error(
                `Received an error response from ${url} with status code ${resp.status}.`
            );
            apiErrorOccurred = true;
            return;
        }
        backendConnected = true;
    });
</script>

{#if apiErrorOccurred}
    <Errors message={"Failed to make initial connection to the backend API."} />
{/if}

{#if currentGameType && isCellsToWinWithinLimits(currentGameType) && currentGameLevel && startGame && backendConnected}
    <Board
        gameType={currentGameType}
        gameLevel={currentGameLevel}
        currentPlayer={getRandomPlayer()}
        endGameFn={endGame}
        goHomeFn={resetGame}
    />
{:else if currentGameType && isCellsToWinWithinLimits(currentGameType) && currentGameLevel && startGame}
    <!-- Wait backend connection -->
    <Spinner />
{:else if showGameEndOptions}
    <div class="open-page" id="game-end-view">
        {#if currentWinner !== undefined}
            <h1>
                Game over:{currentWinner === Player.Bot
                    ? " Bot won!"
                    : currentWinner === Player.P1
                      ? " You won!"
                      : " Tie game!"}
            </h1>
        {:else}
            <h1>Game over</h1>
        {/if}
        {#if currentWinner !== undefined && previousGameLevel?.levelName !== "Easy" && Math.random() > SHOW_PHRASES_LIMIT}
            <h2>{getPhrase(currentWinner)}</h2>
        {/if}
        <button on:click={resetGame} id="game-end-reset">Play again</button>
        <button on:click={backToStart} id="game-end-back">Back to start</button>
    </div>
{:else}
    <div class="open-page" id="new-game-view">
        <h1>Let's play k-in-a-row!</h1>
        {#if showGameInfo}
            <Info {toggleShowGameInfo} />
        {:else}
            <Dropdowns
                {setGameType}
                {setGameLevel}
                {toggleStartGame}
                selectedGame={currentGameType}
                selectedGameLevel={currentGameLevel}
            />
            <div
                on:click={toggleShowGameInfo}
                on:keydown
                role="button"
                tabindex="0"
                class="info-icon"
                id="info-icon"
            >
                <svg
                    xmlns="http://www.w3.org/2000/svg"
                    width="50"
                    height="50"
                    fill="currentColor"
                    class="bi bi-info-circle"
                    viewBox="0 0 16 16"
                >
                    <path
                        d="M8 15A7 7 0 1 1 8 1a7 7 0 0 1 0 14zm0 1A8 8 0 1 0 8 0a8 8 0 0 0 0 16z"
                    />
                    <path
                        d="m8.93 6.588-2.29.287-.082.38.45.083c.294.07.352.176.288.469l-.738 3.468c-.194.897.105 1.319.808 1.319.545 0 1.178-.252 1.465-.598l.088-.416c-.2.176-.492.246-.686.246-.275 0-.375-.193-.304-.533L8.93 6.588zM9 4.5a1 1 0 1 1-2 0 1 1 0 0 1 2 0z"
                    />
                </svg>
            </div>
        {/if}
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
        --cell-size-large-board: 4.25em;
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
        overflow: auto;
        top: 0;
        bottom: 0;
        left: 0;
        right: 0;
    }
    .open-page h1 {
        margin: 2em 0 2.5em;
    }
    .open-page#game-end-view h1 {
        margin: 2em 0 0.75em;
    }
    .open-page#game-end-view h2 {
        margin-bottom: 2em;
        max-width: 70%;
        width: 100%;
        justify-content: center;
        display: flex;
    }
    .open-page button {
        margin: 0.5em;
        margin-bottom: 0.75em;
        padding: 0.5em 1em;
        border: none;
        border-radius: 0.5em;
        background-color: var(--default-light-green);
        color: var(--default-black);
        font-size: 1.5rem;
        font-weight: bold;
        cursor: pointer;
        transition:
            background-color 0.3s,
            color 0.3s;
    }
    .open-page button#game-end-reset {
        margin-bottom: 1.25em;
    }
    .open-page button:hover {
        background-color: var(--default-light-green-hover);
    }
    .info-icon {
        position: absolute;
        bottom: 10%;
        cursor: pointer;
    }
    @media screen and (max-width: 450px) {
        .open-page h1 {
            font-size: 1.75rem;
        }
    }
    @media screen and (max-height: 500px) {
        .info-icon {
            bottom: 1%;
        }
    }
</style>
