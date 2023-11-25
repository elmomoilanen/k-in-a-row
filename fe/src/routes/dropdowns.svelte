<script lang="ts">
    import type { Game, MaybeGame, GameLevel, MaybeGameLevel } from "$lib/games";
    import { Games, GameLevels } from "$lib/games";
    import { range } from "$lib/utils";

    export let setGameType: (game: Game) => void;
    export let setGameLevel: (gameLevel: MaybeGameLevel) => void;
    export let toggleStartGame: () => void;
    export let selectedGame: MaybeGame;
    export let selectedGameLevel: MaybeGameLevel;

    let showGameOptions = false;
    let showGameTypeOptions = false;
    let showGameLevelOptions = false;

    function makeListOfGames() {
        let games = [];
        for (const [key, game] of Object.entries(Games)) {
            games.push({
                id: key,
                game: game
            });
        }
        return games;
    }

    function makeListOfGameLevels() {
        let gameLevels = [];
        for (const [key, level] of Object.entries(GameLevels)) {
            gameLevels.push({ id: `level-${key}`, level: level, description: level.levelName });
        }
        return gameLevels;
    }

    function toggleShowGameOptions() {
        showGameOptions = !showGameOptions;
        showGameTypeOptions = false;
        showGameLevelOptions = false;
    }

    function toggleShowGameTypeOptions() {
        showGameTypeOptions = !showGameTypeOptions;
        showGameOptions = false;
        showGameLevelOptions = false;
    }

    function toggleShowGameLevelOptions() {
        showGameLevelOptions = !showGameLevelOptions;
        showGameOptions = false;
        showGameTypeOptions = false;
    }

    function setGameTypeAndToggle(newGame: Game, newCellsToWin: number) {
        newGame.cellsToWin = newCellsToWin;
        if (newCellsToWin > 0) {
            toggleShowGameTypeOptions();
        } else {
            toggleShowGameOptions();
        }
        setGameType(newGame);
        setGameLevel(undefined);
    }

    function setGameTypeAndToggleSafe(newGame: MaybeGame, newCellsToWin: number) {
        if (newGame) {
            setGameTypeAndToggle(newGame, newCellsToWin);
        }
    }

    function setGameLevelAndToggle(newGameLevel: GameLevel) {
        toggleShowGameLevelOptions();
        setGameLevel(newGameLevel);
    }

    let games = makeListOfGames();
    let gameLevels = makeListOfGameLevels();
</script>

<div class="dropdown" id="game-board">
    <input
        on:click={toggleShowGameOptions}
        type="text"
        id="game-board-input"
        class={selectedGame && !showGameOptions ? "selected" : "not-selected"}
        placeholder={selectedGame ? `${selectedGame.boardSize}` : "Select board size"}
        readonly
    />
    <div class={`options ${showGameOptions ? "show" : ""}`} id="game-boards">
        {#each games as { id, game }}
            <button {id} on:click={() => setGameTypeAndToggle(game, 0)}>{game.boardSize}</button>
        {/each}
    </div>
</div>

{#if selectedGame && !showGameOptions}
    <div class="dropdown" id="game-cells-to-win">
        <input
            on:click={toggleShowGameTypeOptions}
            type="text"
            id="game-cells-to-win-input"
            class={selectedGame.cellsToWin && !showGameTypeOptions ? "selected" : "not-selected"}
            placeholder={selectedGame.cellsToWin
                ? `${selectedGame.cellsToWin}-in-a-row`
                : "Select game type"}
            readonly
        />
        <div class={`options ${showGameTypeOptions ? "show" : ""}`} id="game-types">
            {#each range(selectedGame.cellsToWinMin, selectedGame.cellsToWinMax + 1) as cellsToWin, i}
                <!-- As of now, TS/Svelte language support thinks that `selectedGame` can be undefined here which is false -->
                <!-- Type assertion isn't possible in the Svelte markup, thus use a helper function -->
                <button
                    id={i.toString()}
                    on:click={() => setGameTypeAndToggleSafe(selectedGame, cellsToWin)}
                    >{cellsToWin}-in-a-row</button
                >
            {/each}
        </div>
    </div>
{/if}

{#if selectedGame && selectedGame.cellsToWin && !showGameOptions && !showGameTypeOptions}
    <div class="dropdown" id="game-level">
        <input
            on:click={toggleShowGameLevelOptions}
            type="text"
            id="game-level-input"
            class={selectedGameLevel && !showGameLevelOptions ? "selected" : "not-selected"}
            placeholder={selectedGameLevel ? `${selectedGameLevel.levelName}` : "Select level"}
            readonly
        />
        <div class={`options ${showGameLevelOptions ? "show" : ""}`} id="game-levels">
            {#each gameLevels as { id, level, description }}
                <button {id} on:click={() => setGameLevelAndToggle(level)}>{description}</button>
            {/each}
        </div>
    </div>
{/if}

{#if selectedGame && selectedGame.cellsToWin && !showGameOptions && !showGameTypeOptions && selectedGameLevel && !showGameLevelOptions}
    <button class="start-game-btn" id="start-game" on:click={toggleStartGame}>Start game</button>
{/if}

<style>
    .dropdown#game-board,
    .dropdown#game-cells-to-win {
        overflow-y: scroll;
    }
    .dropdown input {
        position: relative;
        background-color: var(--default-light-gray);
        padding: 1em 0.5em 1em 0.5em;
        border-radius: 10px;
        cursor: pointer;
        border: none;
        outline: none;
    }
    .dropdown#game-cells-to-win,
    .dropdown#game-level {
        margin-top: 25px;
    }
    .dropdown input.not-selected {
        background-color: var(--default-light-gray);
    }
    .dropdown input.not-selected:hover {
        background-color: var(--default-light-gray-hover);
    }
    .dropdown input.not-selected::placeholder {
        color: var(--default-black);
        font-size: 1rem;
        font-weight: bold;
    }
    .dropdown input.selected {
        background-color: var(--default-light-green);
    }
    .dropdown input.selected:hover {
        background-color: var(--default-light-green-hover);
    }
    .dropdown input.selected::placeholder {
        color: var(--default-black);
        font-size: 1rem;
        font-weight: bold;
    }
    .dropdown .options {
        display: none;
    }
    .dropdown .options.show {
        display: flex;
        flex-direction: column;
        margin-top: 10px;
    }
    .dropdown .options button {
        background-color: white;
        padding: 0.75em 0.5em;
        width: 100%;
        border: none;
        outline: none;
        margin-bottom: 0em;
        cursor: pointer;
        font-size: 1rem;
        z-index: 1;
    }
    .dropdown .options button:hover {
        background-color: var(--default-mid-gray-hover);
        color: white;
    }
    .start-game-btn {
        margin-top: 4em;
        font-size: 1.5rem;
        font-weight: bold;
        background-color: var(--default-light-green);
        padding: 0.5em 1em;
        border-radius: 0.5em;
        border: none;
        box-shadow: 0 5px 20px rgba(0, 0, 0, 0.05);
        cursor: pointer;
        color: var(--default-black);
        transition:
            background-color 0.3s,
            color 0.3s;
        position: relative;
        left: 0%;
    }
    .start-game-btn:hover {
        background-color: var(--default-light-green-hover);
        border: 1px solid white;
    }
    @media (prefers-color-scheme: dark) {
        .dropdown input,
        .dropdown input.not-selected {
            background-color: white;
        }
        .dropdown .options button {
            background-color: #1c1c1c;
        }
        .dropdown .options button:hover {
            background-color: var(--default-light-gray-hover);
            color: black;
        }
    }
</style>
