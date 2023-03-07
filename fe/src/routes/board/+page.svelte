<script lang="ts">
    import type { BotMove } from "$lib/botmove";
    import type { Game, GameLevel } from "$lib/games";
    import { Player } from "$lib/players";
    import Winner from "./winner.svelte";
    // @ts-ignore
    import { PUBLIC_API_URL } from "$env/static/public";

    export let gameType: Game;
    export let gameLevel: GameLevel;
    export let currentPlayer: Player;
    export let endGameFn: () => void;

    const X_MARK = "x-symbol";
    const O_MARK = "o-symbol";
    const EMPTY_MARK = "";

    let currentMarker = Math.random() < 0.5 ? X_MARK : O_MARK;
    let gameOver = false;
    let gameWinner = Player.Empty;

    function initCells(cellsCount: number) {
        let cells = [];
        for (let j = 0; j < cellsCount; ++j) {
            cells.push({ id: `${j}`, val: Player.Empty, mark: EMPTY_MARK });
        }
        return cells;
    }

    let cells = initCells(gameType.cellsTotal);

    function changeMarker() {
        currentMarker = currentMarker === X_MARK ? O_MARK : X_MARK;
    }

    function changePlayer() {
        currentPlayer = currentPlayer === Player.P1 ? Player.Bot : Player.P1;
    }

    function playP1Turn(cellId: string) {
        const cell = parseInt(cellId);
        cells[cell] = { id: cellId, val: Player.P1, mark: currentMarker };
        changePlayer();
        changeMarker();
    }

    async function playBotTurn() {
        const level = gameLevel.levelName === "Easy" ? "Easy" : "Normal";
        const url = `${PUBLIC_API_URL}/api/bot/next?level=${level}`;

        const responseRaw = await fetch(url, {
            method: "POST",
            headers: {
                "Content-Type": "application/json",
                Accept: "application/json"
            },
            body: JSON.stringify({
                cells: cells.map((cell) => cell.val),
                p1_mark: Player.P1,
                bot_mark: Player.Bot,
                empty_mark: Player.Empty
            })
        });
        if (!responseRaw.ok) {
            throw new Error(`API error: ${responseRaw.status}`);
        }
        const responseParsed: BotMove = await responseRaw.json();

        if (responseParsed.next_is_valid) {
            const cell = responseParsed.next;
            cells[cell] = { id: `${cell}`, val: Player.Bot, mark: currentMarker };
        }
        gameWinner = responseParsed.winner;
        gameOver = responseParsed.game_over;
        changePlayer();
        changeMarker();
    }
</script>

{#if gameOver}
    <Winner {gameWinner} {endGameFn} />
{/if}

{#if currentPlayer === Player.Bot}
    {#await playBotTurn()}
        <div style="visibility:hidden;display:none" />
    {:catch error}
        <p style="color: red">{error}</p>
    {/await}
{/if}

<div class="board {gameType.gameKey} {currentMarker}" id="board-{gameType.gameKey}">
    {#each cells as { id, val, mark }}
        {#if gameOver && val === Player.Empty && mark === EMPTY_MARK}
            <button class="cell bot" {id} />
        {:else if currentPlayer === Player.P1 && val === Player.Empty && mark === EMPTY_MARK}
            <button class="cell" {id} on:click={() => playP1Turn(id)} />
        {:else if currentPlayer === Player.Bot && val === Player.Empty && mark === EMPTY_MARK}
            <button class="cell bot" {id} />
        {:else}
            <button class="cell {mark}" {id} />
        {/if}
    {/each}
</div>

<style>
    .board {
        width: 100vw;
        height: 100vh;
        display: grid;
        justify-content: center;
        justify-items: center;
        align-content: center;
        align-items: center;
    }
    .cell {
        width: var(--cell-size);
        height: var(--cell-size);
        border: 1px solid black;
        background-color: var(--default-white);
        padding: 0;
        display: flex;
        justify-content: center;
        align-items: center;
        position: relative;
        cursor: pointer;
    }
    @media screen and (max-width: 800px) {
        .cell {
            width: var(--cell-size-small);
            height: var(--cell-size-small);
        }
    }
    @media (prefers-color-scheme: dark) {
        .cell {
            border: 1px solid white;
            background-color: var(--default-black);
        }
    }

    .x33 {
        grid-template-columns: repeat(3, auto);
    }
    .board.x33 .cell:nth-child(3n + 1) {
        border-left: none;
    }
    .board.x33 .cell:nth-child(3n + 3) {
        border-right: none;
    }
    .board.x33 .cell:nth-child(n) {
        border-top: none;
    }
    .board.x33 .cell:nth-child(n + 7) {
        border-bottom: none;
    }

    .x44 {
        grid-template-columns: repeat(4, auto);
    }
    .board.x44 .cell:nth-child(4n + 1) {
        border-left: none;
    }
    .board.x44 .cell:nth-child(4n + 4) {
        border-right: none;
    }
    .board.x44 .cell:nth-child(n) {
        border-top: none;
    }
    .board.x44 .cell:nth-child(n + 13) {
        border-bottom: none;
    }

    .x55 {
        grid-template-columns: repeat(5, auto);
    }
    .board.x55 .cell:nth-child(5n + 1) {
        border-left: none;
    }
    .board.x55 .cell:nth-child(5n + 5) {
        border-right: none;
    }
    .board.x55 .cell:nth-child(n) {
        border-top: none;
    }
    .board.x55 .cell:nth-child(n + 21) {
        border-bottom: none;
    }

    .x66 {
        grid-template-columns: repeat(6, auto);
    }
    .board.x66 .cell:nth-child(6n + 1) {
        border-left: none;
    }
    .board.x66 .cell:nth-child(6n + 6) {
        border-right: none;
    }
    .board.x66 .cell:nth-child(n) {
        border-top: none;
    }
    .board.x66 .cell:nth-child(n + 31) {
        border-bottom: none;
    }

    .x77 {
        grid-template-columns: repeat(7, auto);
    }
    .board.x77 .cell:nth-child(7n + 1) {
        border-left: none;
    }
    .board.x77 .cell:nth-child(7n + 7) {
        border-right: none;
    }
    .board.x77 .cell:nth-child(n) {
        border-top: none;
    }
    .board.x77 .cell:nth-child(n + 43) {
        border-bottom: none;
    }

    .x88 {
        grid-template-columns: repeat(8, auto);
    }
    .board.x88 .cell:nth-child(8n + 1) {
        border-left: none;
    }
    .board.x88 .cell:nth-child(8n + 8) {
        border-right: none;
    }
    .board.x88 .cell:nth-child(n) {
        border-top: none;
    }
    .board.x88 .cell:nth-child(n + 57) {
        border-bottom: none;
    }

    .x99 {
        grid-template-columns: repeat(9, auto);
    }
    .board.x99 .cell:nth-child(9n + 1) {
        border-left: none;
    }
    .board.x99 .cell:nth-child(9n + 9) {
        border-right: none;
    }
    .board.x99 .cell:nth-child(n) {
        border-top: none;
    }
    .board.x99 .cell:nth-child(n + 73) {
        border-bottom: none;
    }

    .cell.x-symbol,
    .cell.o-symbol,
    .cell.bot {
        cursor: not-allowed;
    }
    .cell.x-symbol::before,
    .cell.x-symbol::after,
    .cell.o-symbol::before,
    .cell.o-symbol::after {
        background-color: black;
    }
    .board.x-symbol .cell:not(.x-symbol):not(.o-symbol):not(.bot):hover::before,
    .board.x-symbol .cell:not(.x-symbol):not(.o-symbol):not(.bot):hover::after {
        background-color: var(--default-gray);
    }
    .board.o-symbol .cell:not(.x-symbol):not(.o-symbol):not(.bot):hover::before,
    .board.o-symbol .cell:not(.x-symbol):not(.o-symbol):not(.bot):hover::after {
        background-color: var(--default-gray);
    }
    .cell.x-symbol::before,
    .cell.x-symbol::after,
    .board.x-symbol .cell:not(.x-symbol):not(.o-symbol):not(.bot):hover::before,
    .board.x-symbol .cell:not(.x-symbol):not(.o-symbol):not(.bot):hover::after {
        content: "";
        position: absolute;
        width: calc(var(--mark-size) * 0.1);
        height: calc(var(--mark-size) * 1);
    }
    .cell.x-symbol::before,
    .board.x-symbol .cell:not(.x-symbol):not(.o-symbol):not(.bot):hover::before {
        transform: rotate(45deg);
    }
    .cell.x-symbol::after,
    .board.x-symbol .cell:not(.x-symbol):not(.o-symbol):not(.bot):hover::after {
        transform: rotate(-45deg);
    }
    .cell.o-symbol::before,
    .cell.o-symbol::after,
    .board.o-symbol .cell:not(.x-symbol):not(.o-symbol):not(.bot):hover::before,
    .board.o-symbol .cell:not(.x-symbol):not(.o-symbol):not(.bot):hover::after {
        content: "";
        position: absolute;
        border-radius: 50%;
    }
    .cell.o-symbol::before,
    .board.o-symbol .cell:not(.x-symbol):not(.o-symbol):not(.bot):hover::before {
        width: var(--mark-size);
        height: var(--mark-size);
    }
    .cell.o-symbol::after,
    .board.o-symbol .cell:not(.x-symbol):not(.o-symbol):not(.bot):hover::after {
        width: calc(var(--mark-size) * 0.8);
        height: calc(var(--mark-size) * 0.8);
        background-color: var(--default-white);
    }

    @media screen and (max-width: 800px) {
        .cell.x-symbol::before,
        .cell.x-symbol::after,
        .board.x-symbol .cell:not(.x-symbol):not(.o-symbol):not(.bot):hover::before,
        .board.x-symbol .cell:not(.x-symbol):not(.o-symbol):not(.bot):hover::after {
            width: calc(var(--mark-size-small) * 0.1);
            height: calc(var(--mark-size-small) * 1);
        }
        .cell.o-symbol::before,
        .board.o-symbol .cell:not(.x-symbol):not(.o-symbol):not(.bot):hover::before {
            width: var(--mark-size-small);
            height: var(--mark-size-small);
        }
        .cell.o-symbol::after,
        .board.o-symbol .cell:not(.x-symbol):not(.o-symbol):not(.bot):hover::after {
            width: calc(var(--mark-size-small) * 0.8);
            height: calc(var(--mark-size-small) * 0.8);
        }
    }
    @media (prefers-color-scheme: dark) {
        .cell.x-symbol::before,
        .cell.x-symbol::after,
        .cell.o-symbol::before,
        .cell.o-symbol::after {
            background-color: white;
        }
        .cell.o-symbol::after,
        .board.o-symbol .cell:not(.x-symbol):not(.o-symbol):not(.bot):hover::after {
            background-color: var(--default-black);
        }
    }
</style>
