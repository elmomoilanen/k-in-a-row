<script lang="ts">
    import type { BotMove } from "$lib/botmove";
    import type { Game, GameLevel } from "$lib/games";
    import { Player } from "$lib/players";
    import { hasWinner } from "$lib/winner";
    import { sleep } from "$lib/utils";
    import Start from "./start.svelte";
    import Topbar from "./topbar.svelte";
    import Errors from "../errors.svelte";
    import { PUBLIC_API_URL } from "$env/static/public";

    // Default value is only for prerendering, otherwise always pass the correct game type as props
    export let gameType: Game = {
        cellsTotal: 9,
        cellsToWin: 3,
        cellsToWinMin: 3,
        cellsToWinMax: 3,
        boardSize: "3x3",
        gameKey: "x33"
    };
    export let gameLevel: GameLevel;
    export let currentPlayer: Player;
    export let endGameFn: (winner: Player) => void;
    export let goHomeFn: () => void;

    const X_MARK = "x-symbol";
    const O_MARK = "o-symbol";
    const EMPTY_MARK = "";
    const HIGHLIGHT_MIN_BOARD_SIZE = 100;
    const HIGHLIGHT_BOT_MOVE_MS = 1000;
    const WAIT_AFTER_END_BEFORE_DESTROY_BOARD_MS = 2500;
    const SHOW_P1_START_NOTIFICATION_MS = 1000;
    const BE_REQUESTS_MAX_TRIES = 5;
    const BE_REQUESTS_RETRY_DELAY_MS = 1000;

    let currentMarker = Math.random() < 0.5 ? X_MARK : O_MARK;
    let botPlayerLastSelectedCell: string | undefined = undefined;
    let p1LastMoveCell = -1;
    let botLastMoveCell = -1;
    let showStart = currentPlayer === Player.P1;
    let gameOver = false;
    let gameWinner = Player.Empty;

    function initCells(cellsCount: number) {
        const cells = [];
        for (let j = 0; j < cellsCount; ++j) {
            cells.push({ id: `${j}`, value: Player.Empty, mark: EMPTY_MARK });
        }
        return cells;
    }

    let cells = initCells(gameType.cellsTotal);

    function highlightBotMove(botLastMoveCell: string) {
        botPlayerLastSelectedCell = undefined;
        const botLastMoveCellIdx = parseInt(botLastMoveCell);
        const lastSelectedCell = document.querySelector(`[id='${botLastMoveCellIdx}']`);
        if (lastSelectedCell) {
            lastSelectedCell.classList.add("bot-last-move-cell");
            setTimeout(() => {
                lastSelectedCell.classList.remove("bot-last-move-cell");
            }, HIGHLIGHT_BOT_MOVE_MS);
        }
    }

    function changeMarker() {
        currentMarker = currentMarker === X_MARK ? O_MARK : X_MARK;
    }

    function changePlayer() {
        currentPlayer = currentPlayer === Player.P1 ? Player.Bot : Player.P1;
    }

    function playP1Turn(cellId: string) {
        const cell = parseInt(cellId);
        p1LastMoveCell = cell;
        botPlayerLastSelectedCell = undefined;
        cells[cell] = { id: cellId, value: Player.P1, mark: currentMarker };
        changePlayer();
        changeMarker();
    }

    async function requestBotMove(url: string) {
        try {
            return await fetch(url, {
                method: "POST",
                headers: {
                    "Content-Type": "application/json",
                    Accept: "application/json"
                },
                body: JSON.stringify({
                    cells: cells.map((cell) => cell.value),
                    cells_to_win: gameType.cellsToWin,
                    p1_mark: Player.P1,
                    bot_mark: Player.Bot,
                    empty_mark: Player.Empty
                })
            });
        } catch (error) {
            console.error(error);
            return undefined;
        }
    }

    async function playBotTurn() {
        const level = gameLevel.levelName === "Easy" ? "Easy" : "Normal";
        const url = `${PUBLIC_API_URL}/api/bot/next?level=${level}`;

        let responseParsed: BotMove | undefined = undefined;

        for (let j = 1; j <= BE_REQUESTS_MAX_TRIES; ++j) {
            const responseRaw = await requestBotMove(url);
            if (responseRaw?.ok) {
                responseParsed = await responseRaw.json();
                break;
            }
            if (j >= BE_REQUESTS_MAX_TRIES) {
                throw new Error("Failed to get proper response from the backend API call.");
            }
            await sleep(BE_REQUESTS_RETRY_DELAY_MS * j);
        }

        if (responseParsed === undefined || Object.keys(responseParsed).length === 0) {
            // It's a logic error if we get here
            throw new Error("Failed to get proper response from the backend API call.");
        }

        if (responseParsed.next_is_valid) {
            const cell = responseParsed.next;
            botLastMoveCell = cell;
            botPlayerLastSelectedCell = `${cell}`;
            cells[cell] = { id: botPlayerLastSelectedCell, value: Player.Bot, mark: currentMarker };
        }

        gameWinner = responseParsed.winner;
        gameOver = responseParsed.game_over;
        changePlayer();
        changeMarker();
    }

    function completeGame() {
        if (gameWinner !== Player.Empty) {
            const lastMoveCell = gameWinner === Player.Bot ? botLastMoveCell : p1LastMoveCell;
            const cellsOffset = parseInt(
                gameType.boardSize.substring(0, gameType.boardSize.indexOf("x"))
            );
            let result =
                cellsOffset &&
                hasWinner(
                    { cells: cells, cellsOffset: cellsOffset, cellsToWin: gameType.cellsToWin },
                    lastMoveCell,
                    gameWinner
                );
            if (result && result.hasWinner && result.winCells) {
                for (let i = 0; i < result.winCells.length; ++i) {
                    const boardCell = document.querySelector(`[id='${result.winCells[i]}']`);
                    if (boardCell) {
                        boardCell.classList.add(`${gameWinner === Player.Bot ? "lost" : "win"}`);
                    }
                }
            }
        }
        setTimeout(() => {
            endGameFn(gameWinner);
        }, WAIT_AFTER_END_BEFORE_DESTROY_BOARD_MS);
    }
</script>

{#if showStart}
    <Start bind:showStart showTime={SHOW_P1_START_NOTIFICATION_MS} />
{/if}

{#if !gameOver}
    <Topbar boardSize={gameType.boardSize} cellsToWin={gameType.cellsToWin} goHome={goHomeFn} />
{/if}

{#if !gameOver && currentPlayer === Player.Bot}
    {#await playBotTurn()}
        <div style="visibility:hidden;display:none" />
    {:catch error}
        <Errors message={error} />
    {/await}
{/if}

<div class="board {gameType.gameKey} {currentMarker}" id="board-{gameType.gameKey}">
    {#each cells as { id, value, mark }}
        {#if gameOver && value === Player.Empty && mark === EMPTY_MARK}
            <button class="cell bot" {id} />
        {:else if currentPlayer === Player.P1 && value === Player.Empty && mark === EMPTY_MARK}
            <button class="cell" {id} on:click={() => playP1Turn(id)} />
        {:else if currentPlayer === Player.Bot && value === Player.Empty && mark === EMPTY_MARK}
            <button class="cell bot" {id} />
        {:else}
            <button class="cell {mark}" {id} />
        {/if}
    {/each}
</div>

{#if gameType.cellsTotal >= HIGHLIGHT_MIN_BOARD_SIZE && !gameOver && currentPlayer === Player.P1 && botPlayerLastSelectedCell}
    {highlightBotMove(botPlayerLastSelectedCell)}
{/if}

{#if gameOver}
    <div style="visibility:hidden;display:none">{completeGame()}</div>
{/if}

<style>
    .board {
        width: 100vw;
        height: 100vh;
        display: grid;
        justify-content: center;
        justify-items: center;
        align-content: center;
        align-items: center;
        overflow: auto;
        scroll-snap-type: x mandatory;
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
        box-sizing: border-box;
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
    .bot-last-move-cell {
        animation: bot-move-highlight 0.5s ease-in-out;
    }
    @keyframes bot-move-highlight {
        0% {
            transform: scale(1);
        }
        50% {
            transform: scale(1.05);
        }
        100% {
            transform: scale(1);
        }
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
        .board.x77,
        .board.x88,
        .board.x99,
        .board.x1010,
        .board.x1111,
        .board.x1212,
        .board.x1313,
        .board.x1414,
        .board.x1515 {
            justify-content: flex-start;
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
    .cell.lost.x-symbol::before,
    .cell.lost.x-symbol::after,
    .cell.lost.o-symbol::before,
    .cell.lost.o-symbol::after {
        background-color: red;
    }
    .cell.win.x-symbol::before,
    .cell.win.x-symbol::after,
    .cell.win.o-symbol::before,
    .cell.win.o-symbol::after {
        background-color: rgb(0, 180, 0);
    }
    .cell.lost.o-symbol::after,
    .cell.win.o-symbol::after {
        background-color: var(--default-white);
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
        .cell.lost.x-symbol::before,
        .cell.lost.x-symbol::after,
        .cell.lost.o-symbol::before,
        .cell.lost.o-symbol::after {
            background-color: #b53b3b;
        }
        .cell.win.x-symbol::before,
        .cell.win.x-symbol::after,
        .cell.win.o-symbol::before,
        .cell.win.o-symbol::after {
            background-color: #50b53b;
        }
        .cell.lost.o-symbol::after,
        .cell.win.o-symbol::after {
            background-color: var(--default-black);
        }
    }
</style>
