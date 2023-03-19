<script lang="ts">
    import type { Game, MaybeGame, GameLevel, MaybeGameLevel } from "$lib/games";
    import { Games, GameLevels } from "$lib/games";

    export let setGameType: (game: Game) => void;
    export let setGameLevel: (gameLevel: MaybeGameLevel) => void;
    export let toggleStartGame: () => void;
    export let selectedGameType: MaybeGame;
    export let selectedGameLevel: MaybeGameLevel;

    let showGameTypeOptions = false;
    let showGameLevelOptions = false;

    function makeListOfGames() {
        let games = [];
        let j = 0;
        for (const [key, game] of Object.entries(Games)) {
            const description_suffix =
                j == 0 ? `(${game.cellsToWin}-in-a-row)` : `(${game.cellsToWin})`;
            games.push({
                id: key,
                game: game,
                description: `${game.boardSize} ${description_suffix}`
            });
            j += 1;
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

    function toggleShowGameTypeOptions() {
        showGameTypeOptions = !showGameTypeOptions;
        showGameLevelOptions = false;
    }

    function toggleShowGameLevelOptions() {
        showGameLevelOptions = !showGameLevelOptions;
        showGameTypeOptions = false;
    }

    function setGameTypeAndToggle(newGame: Game) {
        toggleShowGameTypeOptions();
        setGameType(newGame);
        setGameLevel(undefined);
    }

    function setGameLevelAndToggle(newGameLevel: GameLevel) {
        toggleShowGameLevelOptions();
        setGameLevel(newGameLevel);
    }

    let games = makeListOfGames();
    let gameLevels = makeListOfGameLevels();
</script>

<div class="dropdown" id="game-type">
    <input
        on:click={toggleShowGameTypeOptions}
        type="text"
        class={selectedGameType && !showGameTypeOptions ? "selected" : "not-selected"}
        placeholder={selectedGameType
            ? `${selectedGameType.boardSize} ${selectedGameType.cellsToWin}-in-a-row`
            : "Select game type"}
        readonly
    />
    <div class={`options ${showGameTypeOptions ? "show" : ""}`} id="game-types">
        {#each games as { id, game, description }}
            <button {id} on:click={() => setGameTypeAndToggle(game)}>{description}</button>
        {/each}
    </div>
</div>

{#if selectedGameType && !showGameTypeOptions}
    <div class="dropdown" id="game-level">
        <input
            on:click={toggleShowGameLevelOptions}
            type="text"
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

{#if selectedGameType && !showGameTypeOptions && selectedGameLevel && !showGameLevelOptions}
    <button class="start-game-btn" on:click={toggleStartGame}>Start game</button>
{/if}

<style>
    .dropdown#game-type {
        overflow-y: auto;
    }
    .dropdown input {
        position: relative;
        background-color: var(--default-light-gray);
        padding: 1em 0 1em 0.5em;
        border-radius: 10px;
        cursor: pointer;
        border: none;
        outline: none;
    }
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
        font-size: 1.2rem;
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
        font-size: 1.2rem;
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
        padding: 0.5em 1.5em;
        border-radius: 0.5em;
        border: none;
        box-shadow: 0 5px 20px rgba(0, 0, 0, 0.05);
        cursor: pointer;
        color: var(--default-black);
        transition: background-color 0.3s, color 0.3s;
        position: relative;
        left: 0.5%;
    }
    .start-game-btn:hover {
        background-color: var(--default-light-green-hover);
        border: 1px solid white;
    }
    @media (prefers-color-scheme: dark) {
        .dropdown input {
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
