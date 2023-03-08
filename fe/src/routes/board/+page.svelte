<script lang="ts">
    import type { BotMove } from "$lib/botmove";
    import type { Game, GameLevel } from "$lib/games";
    import { Player } from "$lib/players";
    import Winner from "./winner.svelte";
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
    .board.x1010 .cell,
    .board.x1111 .cell,
    .board.x1212 .cell,
    .board.x1313 .cell,
    .board.x1414 .cell,
    .board.x1515 .cell {
        width: var(--cell-size-large-board);
        height: var(--cell-size-large-board);
    }
    @media screen and (max-width: 800px) {
        .cell {
            width: var(--cell-size-mobile-small-board);
            height: var(--cell-size-mobile-small-board);
        }
        .board.x77 .cell,
        .board.x88 .cell,
        .board.x99 .cell {
            width: var(--cell-size-mobile-mid-board);
            height: var(--cell-size-mobile-mid-board);
        }
        .board.x1010 .cell,
        .board.x1111 .cell,
        .board.x1212 .cell,
        .board.x1313 .cell,
        .board.x1414 .cell,
        .board.x1515 .cell {
            width: var(--cell-size-mobile-large-board);
            height: var(--cell-size-mobile-large-board);
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

    .x1010 {
        grid-template-columns: repeat(10, auto);
    }
    .board.x1010 .cell:nth-child(10n + 1) {
        border-left: none;
    }
    .board.x1010 .cell:nth-child(10n + 10) {
        border-right: none;
    }
    .board.x1010 .cell:nth-child(n) {
        border-top: none;
    }
    .board.x1010 .cell:nth-child(n + 91) {
        border-bottom: none;
    }

    .x1111 {
        grid-template-columns: repeat(11, auto);
    }
    .board.x1111 .cell:nth-child(11n + 1) {
        border-left: none;
    }
    .board.x1111 .cell:nth-child(11n + 11) {
        border-right: none;
    }
    .board.x1111 .cell:nth-child(n) {
        border-top: none;
    }
    .board.x1111 .cell:nth-child(n + 111) {
        border-bottom: none;
    }

    .x1212 {
        grid-template-columns: repeat(12, auto);
    }
    .board.x1212 .cell:nth-child(12n + 1) {
        border-left: none;
    }
    .board.x1212 .cell:nth-child(12n + 12) {
        border-right: none;
    }
    .board.x1212 .cell:nth-child(n) {
        border-top: none;
    }
    .board.x1212 .cell:nth-child(n + 133) {
        border-bottom: none;
    }

    .x1313 {
        grid-template-columns: repeat(13, auto);
    }
    .board.x1313 .cell:nth-child(13n + 1) {
        border-left: none;
    }
    .board.x1313 .cell:nth-child(13n + 13) {
        border-right: none;
    }
    .board.x1313 .cell:nth-child(n) {
        border-top: none;
    }
    .board.x1313 .cell:nth-child(n + 157) {
        border-bottom: none;
    }

    .x1414 {
        grid-template-columns: repeat(14, auto);
    }
    .board.x1414 .cell:nth-child(14n + 1) {
        border-left: none;
    }
    .board.x1414 .cell:nth-child(14n + 14) {
        border-right: none;
    }
    .board.x1414 .cell:nth-child(n) {
        border-top: none;
    }
    .board.x1414 .cell:nth-child(n + 183) {
        border-bottom: none;
    }

    .x1515 {
        grid-template-columns: repeat(15, auto);
    }
    .board.x1515 .cell:nth-child(15n + 1) {
        border-left: none;
    }
    .board.x1515 .cell:nth-child(15n + 15) {
        border-right: none;
    }
    .board.x1515 .cell:nth-child(n) {
        border-top: none;
    }
    .board.x1515 .cell:nth-child(n + 211) {
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
    .board.x1010 .cell.x-symbol::before,
    .board.x1010 .cell.x-symbol::after,
    .board.x1010.x-symbol .cell:not(.x-symbol):not(.o-symbol):not(.bot):hover::before,
    .board.x1010.x-symbol .cell:not(.x-symbol):not(.o-symbol):not(.bot):hover::after,
    .board.x1111 .cell.x-symbol::before,
    .board.x1111 .cell.x-symbol::after,
    .board.x1111.x-symbol .cell:not(.x-symbol):not(.o-symbol):not(.bot):hover::before,
    .board.x1111.x-symbol .cell:not(.x-symbol):not(.o-symbol):not(.bot):hover::after,
    .board.x1212 .cell.x-symbol::before,
    .board.x1212 .cell.x-symbol::after,
    .board.x1212.x-symbol .cell:not(.x-symbol):not(.o-symbol):not(.bot):hover::before,
    .board.x1212.x-symbol .cell:not(.x-symbol):not(.o-symbol):not(.bot):hover::after,
    .board.x1313 .cell.x-symbol::before,
    .board.x1313 .cell.x-symbol::after,
    .board.x1313.x-symbol .cell:not(.x-symbol):not(.o-symbol):not(.bot):hover::before,
    .board.x1313.x-symbol .cell:not(.x-symbol):not(.o-symbol):not(.bot):hover::after,
    .board.x1414 .cell.x-symbol::before,
    .board.x1414 .cell.x-symbol::after,
    .board.x1414.x-symbol .cell:not(.x-symbol):not(.o-symbol):not(.bot):hover::before,
    .board.x1414.x-symbol .cell:not(.x-symbol):not(.o-symbol):not(.bot):hover::after,
    .board.x1515 .cell.x-symbol::before,
    .board.x1515 .cell.x-symbol::after,
    .board.x1515.x-symbol .cell:not(.x-symbol):not(.o-symbol):not(.bot):hover::before,
    .board.x1515.x-symbol .cell:not(.x-symbol):not(.o-symbol):not(.bot):hover::after {
        width: calc(var(--mark-size-large-board) * 0.1);
        height: calc(var(--mark-size-large-board) * 1);
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
    .board.x1010 .cell.o-symbol::before,
    .board.x1010.o-symbol .cell:not(.x-symbol):not(.o-symbol):not(.bot):hover::before,
    .board.x1111 .cell.o-symbol::before,
    .board.x1111.o-symbol .cell:not(.x-symbol):not(.o-symbol):not(.bot):hover::before,
    .board.x1212 .cell.o-symbol::before,
    .board.x1212.o-symbol .cell:not(.x-symbol):not(.o-symbol):not(.bot):hover::before,
    .board.x1313 .cell.o-symbol::before,
    .board.x1313.o-symbol .cell:not(.x-symbol):not(.o-symbol):not(.bot):hover::before,
    .board.x1414 .cell.o-symbol::before,
    .board.x1414.o-symbol .cell:not(.x-symbol):not(.o-symbol):not(.bot):hover::before,
    .board.x1515 .cell.o-symbol::before,
    .board.x1515.o-symbol .cell:not(.x-symbol):not(.o-symbol):not(.bot):hover::before {
        width: var(--mark-size-large-board);
        height: var(--mark-size-large-board);
    }
    .cell.o-symbol::after,
    .board.o-symbol .cell:not(.x-symbol):not(.o-symbol):not(.bot):hover::after {
        width: calc(var(--mark-size) * 0.8);
        height: calc(var(--mark-size) * 0.8);
        background-color: var(--default-white);
    }
    .board.x1010 .cell.o-symbol::after,
    .board.x1010.o-symbol .cell:not(.x-symbol):not(.o-symbol):not(.bot):hover::after,
    .board.x1111 .cell.o-symbol::after,
    .board.x1111.o-symbol .cell:not(.x-symbol):not(.o-symbol):not(.bot):hover::after,
    .board.x1212 .cell.o-symbol::after,
    .board.x1212.o-symbol .cell:not(.x-symbol):not(.o-symbol):not(.bot):hover::after,
    .board.x1313 .cell.o-symbol::after,
    .board.x1313.o-symbol .cell:not(.x-symbol):not(.o-symbol):not(.bot):hover::after,
    .board.x1414 .cell.o-symbol::after,
    .board.x1414.o-symbol .cell:not(.x-symbol):not(.o-symbol):not(.bot):hover::after,
    .board.x1515 .cell.o-symbol::after,
    .board.x1515.o-symbol .cell:not(.x-symbol):not(.o-symbol):not(.bot):hover::after {
        width: calc(var(--mark-size-large-board) * 0.8);
        height: calc(var(--mark-size-large-board) * 0.8);
    }

    @media screen and (max-width: 800px) {
        .cell.x-symbol::before,
        .cell.x-symbol::after,
        .board.x-symbol .cell:not(.x-symbol):not(.o-symbol):not(.bot):hover::before,
        .board.x-symbol .cell:not(.x-symbol):not(.o-symbol):not(.bot):hover::after {
            width: calc(var(--mark-size-mobile-small-board) * 0.1);
            height: calc(var(--mark-size-mobile-small-board) * 1);
        }
        .cell.o-symbol::before,
        .board.o-symbol .cell:not(.x-symbol):not(.o-symbol):not(.bot):hover::before {
            width: var(--mark-size-mobile-small-board);
            height: var(--mark-size-mobile-small-board);
        }
        .cell.o-symbol::after,
        .board.o-symbol .cell:not(.x-symbol):not(.o-symbol):not(.bot):hover::after {
            width: calc(var(--mark-size-mobile-small-board) * 0.8);
            height: calc(var(--mark-size-mobile-small-board) * 0.8);
        }
        .board.x77 .cell.x-symbol::before,
        .board.x88 .cell.x-symbol::before,
        .board.x99 .cell.x-symbol::before,
        .board.x77 .cell.x-symbol::after,
        .board.x88 .cell.x-symbol::after,
        .board.x99 .cell.x-symbol::after,
        .board.x77.x-symbol .cell:not(.x-symbol):not(.o-symbol):not(.bot):hover::before,
        .board.x88.x-symbol .cell:not(.x-symbol):not(.o-symbol):not(.bot):hover::before,
        .board.x99.x-symbol .cell:not(.x-symbol):not(.o-symbol):not(.bot):hover::before,
        .board.x77.x-symbol .cell:not(.x-symbol):not(.o-symbol):not(.bot):hover::after,
        .board.x88.x-symbol .cell:not(.x-symbol):not(.o-symbol):not(.bot):hover::after,
        .board.x99.x-symbol .cell:not(.x-symbol):not(.o-symbol):not(.bot):hover::after {
            width: calc(var(--mark-size-mobile-mid-board) * 0.1);
            height: calc(var(--mark-size-mobile-mid-board) * 1);
        }
        .board.x77 .cell.o-symbol::before,
        .board.x88 .cell.o-symbol::before,
        .board.x99 .cell.o-symbol::before,
        .board.x77.o-symbol .cell:not(.x-symbol):not(.o-symbol):not(.bot):hover::before,
        .board.x88.o-symbol .cell:not(.x-symbol):not(.o-symbol):not(.bot):hover::before,
        .board.x99.o-symbol .cell:not(.x-symbol):not(.o-symbol):not(.bot):hover::before {
            width: var(--mark-size-mobile-mid-board);
            height: var(--mark-size-mobile-mid-board);
        }
        .board.x77 .cell.o-symbol::after,
        .board.x88 .cell.o-symbol::after,
        .board.x99 .cell.o-symbol::after,
        .board.x77.o-symbol .cell:not(.x-symbol):not(.o-symbol):not(.bot):hover::after,
        .board.x88.o-symbol .cell:not(.x-symbol):not(.o-symbol):not(.bot):hover::after,
        .board.x99.o-symbol .cell:not(.x-symbol):not(.o-symbol):not(.bot):hover::after {
            width: calc(var(--mark-size-mobile-mid-board) * 0.8);
            height: calc(var(--mark-size-mobile-mid-board) * 0.8);
        }
        .board.x1010 .cell.x-symbol::before,
        .board.x1111 .cell.x-symbol::before,
        .board.x1212 .cell.x-symbol::before,
        .board.x1313 .cell.x-symbol::before,
        .board.x1414 .cell.x-symbol::before,
        .board.x1515 .cell.x-symbol::before,
        .board.x1010 .cell.x-symbol::after,
        .board.x1111 .cell.x-symbol::after,
        .board.x1212 .cell.x-symbol::after,
        .board.x1313 .cell.x-symbol::after,
        .board.x1414 .cell.x-symbol::after,
        .board.x1515 .cell.x-symbol::after,
        .board.x1010.x-symbol .cell:not(.x-symbol):not(.o-symbol):not(.bot):hover::before,
        .board.x1111.x-symbol .cell:not(.x-symbol):not(.o-symbol):not(.bot):hover::before,
        .board.x1212.x-symbol .cell:not(.x-symbol):not(.o-symbol):not(.bot):hover::before,
        .board.x1313.x-symbol .cell:not(.x-symbol):not(.o-symbol):not(.bot):hover::before,
        .board.x1414.x-symbol .cell:not(.x-symbol):not(.o-symbol):not(.bot):hover::before,
        .board.x1515.x-symbol .cell:not(.x-symbol):not(.o-symbol):not(.bot):hover::before,
        .board.x1010.x-symbol .cell:not(.x-symbol):not(.o-symbol):not(.bot):hover::after,
        .board.x1111.x-symbol .cell:not(.x-symbol):not(.o-symbol):not(.bot):hover::after,
        .board.x1212.x-symbol .cell:not(.x-symbol):not(.o-symbol):not(.bot):hover::after,
        .board.x1313.x-symbol .cell:not(.x-symbol):not(.o-symbol):not(.bot):hover::after,
        .board.x1414.x-symbol .cell:not(.x-symbol):not(.o-symbol):not(.bot):hover::after,
        .board.x1515.x-symbol .cell:not(.x-symbol):not(.o-symbol):not(.bot):hover::after {
            width: calc(var(--mark-size-mobile-large-board) * 0.1);
            height: calc(var(--mark-size-mobile-large-board) * 1);
        }
        .board.x1010 .cell.o-symbol::before,
        .board.x1111 .cell.o-symbol::before,
        .board.x1212 .cell.o-symbol::before,
        .board.x1313 .cell.o-symbol::before,
        .board.x1414 .cell.o-symbol::before,
        .board.x1515 .cell.o-symbol::before,
        .board.x1010.o-symbol .cell:not(.x-symbol):not(.o-symbol):not(.bot):hover::before,
        .board.x1111.o-symbol .cell:not(.x-symbol):not(.o-symbol):not(.bot):hover::before,
        .board.x1212.o-symbol .cell:not(.x-symbol):not(.o-symbol):not(.bot):hover::before,
        .board.x1313.o-symbol .cell:not(.x-symbol):not(.o-symbol):not(.bot):hover::before,
        .board.x1414.o-symbol .cell:not(.x-symbol):not(.o-symbol):not(.bot):hover::before,
        .board.x1515.o-symbol .cell:not(.x-symbol):not(.o-symbol):not(.bot):hover::before {
            width: var(--mark-size-mobile-large-board);
            height: var(--mark-size-mobile-large-board);
        }
        .board.x1010 .cell.o-symbol::after,
        .board.x1111 .cell.o-symbol::after,
        .board.x1212 .cell.o-symbol::after,
        .board.x1313 .cell.o-symbol::after,
        .board.x1414 .cell.o-symbol::after,
        .board.x1515 .cell.o-symbol::after,
        .board.x1010.o-symbol .cell:not(.x-symbol):not(.o-symbol):not(.bot):hover::after,
        .board.x1111.o-symbol .cell:not(.x-symbol):not(.o-symbol):not(.bot):hover::after,
        .board.x1212.o-symbol .cell:not(.x-symbol):not(.o-symbol):not(.bot):hover::after,
        .board.x1313.o-symbol .cell:not(.x-symbol):not(.o-symbol):not(.bot):hover::after,
        .board.x1414.o-symbol .cell:not(.x-symbol):not(.o-symbol):not(.bot):hover::after,
        .board.x1515.o-symbol .cell:not(.x-symbol):not(.o-symbol):not(.bot):hover::after {
            width: calc(var(--mark-size-mobile-large-board) * 0.8);
            height: calc(var(--mark-size-mobile-large-board) * 0.8);
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
