<script lang="ts">
    import type { BotMove } from "$lib/botmove";
    import { GameType } from "$lib/games";
    import { GameLevel } from "$lib/levels";
    import { Player } from "$lib/players";

    type VoidFn = () => void;

    export let size: number;
    export let gameLevel: GameLevel;
    export let currentPlayer: Player;
    export let endGameFn: VoidFn;

    const X_MARK = "x-symbol";
    const O_MARK = "o-symbol";
    const EMPTY_MARK = "";

    let currentMarker = (Math.random() < 0.5) ? X_MARK : O_MARK;
    let gameOver = false;
    let gameWinner = Player.Empty;

    function initCells(cellsCount: number) {
        let cells = [];
        for (let j=0; j<cellsCount; ++j) {
            cells.push({id: `${j}`, val: Player.Empty, mark: EMPTY_MARK});
        }
        return cells;
    }

    let cells = initCells(size);

    function changeMarker() {
        if (currentMarker === X_MARK) {
            currentMarker = O_MARK;
        } else {
            currentMarker = X_MARK;
        }
    }
    function changePlayer() {
        if (currentPlayer === Player.P1) {
            currentPlayer = Player.Bot;
        } else {
            currentPlayer = Player.P1;
        }
    }

    function playP1Turn(cellId: string) {
        const cell = parseInt(cellId);
        cells[cell] = {id: cellId, val: Player.P1, mark: currentMarker};
        changePlayer();
        changeMarker();
    }

    async function playBotTurn() {
        const API_URL = "http://localhost:8080/api/bot/next";

        // Map level "Hard" to "Normal" for the API call
        const level = (gameLevel === GameLevel.Easy) ? "Easy" : "Normal";
        const url = API_URL + `?level=${level}`;

        const responseRaw = await fetch(url, {
            method: "POST",
            headers: {
                "Content-Type": "application/json",
                "Accept": "application/json",
                "Access-Control-Allow-Origin": "*"
            },
            body: JSON.stringify({
                "cells": cells.map(cell => cell.val),
                "p1_mark": Player.P1,
                "bot_mark": Player.Bot,
                "empty_mark": Player.Empty
            }),
        });
        if (!responseRaw.ok) {
            throw new Error(`API error: ${responseRaw.status}`);
        }
        const responseParsed: BotMove = await responseRaw.json();

        if (responseParsed.next_is_valid) {
            const cell = responseParsed.next;
            cells[cell] = {id: `${cell}`, val: Player.Bot, mark: currentMarker};
        }
        gameWinner = responseParsed.winner;
        gameOver = responseParsed.game_over;
        changePlayer();
        changeMarker();
    }
</script>

{#if gameOver }
    <p>Game winner {gameWinner}</p>
{/if}

{#if currentPlayer === Player.Bot}
    {#await playBotTurn()}
        <p></p>
    {:catch error}
        <p style="color: red">ERROR {error}</p>
    {/await}
{/if}


{#if cells.length === GameType.X33}
    <div class="board x33 {currentMarker}" id="board-x33">
        {#each cells as { id, val, mark }}
            {#if currentPlayer === Player.P1 && val === Player.Empty && mark === EMPTY_MARK}
                <button class="cell" id={id} on:click={() => playP1Turn(id)} />
            {:else}
                <button class="cell {mark}" id={id} />
            {/if}
        {/each}    
    </div>
{:else if cells.length === GameType.X44}
    <div class="board x44 {currentMarker}" id="board-x44">
        {#each cells as { id, val, mark }}
            {#if currentPlayer === Player.P1 && val === Player.Empty && mark === EMPTY_MARK}
                <button class="cell" id={id} on:click={() => playP1Turn(id)} />
            {:else}
                <button class="cell {mark}" id={id} />
            {/if}    
        {/each}            
    </div>
{:else if cells.length === GameType.X55}
    <div class="board x55 {currentMarker}" id="board-x55">
        {#each cells as { id, val, mark }}
            {#if currentPlayer === Player.P1 && val === Player.Empty && mark === EMPTY_MARK}
                <button class="cell" id={id} on:click={() => playP1Turn(id)} />
            {:else}
                <button class="cell {mark}" id={id} />
            {/if}
        {/each}
    </div>
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
    }
    .cell {
        width: var(--cell-size);
        height: var(--cell-size);
        border: 1px solid black;
        background-color: white;
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

    .cell.x-symbol,
    .cell.o-symbol {
        cursor: not-allowed;
    }
    .cell.x-symbol::before,
    .cell.x-symbol::after,
    .cell.o-symbol::before,
    .cell.o-symbol::after {
        background-color: black;
    }
    .board.x-symbol .cell:not(.x-symbol):not(.o-symbol):hover::before,
    .board.x-symbol .cell:not(.x-symbol):not(.o-symbol):hover::after {
        background-color: grey;
    }
    .board.o-symbol .cell:not(.x-symbol):not(.o-symbol):hover::before,
    .board.o-symbol .cell:not(.x-symbol):not(.o-symbol):hover::after {
        background-color: grey;
    }
    .cell.x-symbol::before,
    .cell.x-symbol::after,
    .board.x-symbol .cell:not(.x-symbol):not(.o-symbol):hover::before,
    .board.x-symbol .cell:not(.x-symbol):not(.o-symbol):hover::after {
        content: "";
        position: absolute;
        width: calc(var(--mark-size) * 0.1);
        height: calc(var(--mark-size) * 1);
    }
    .cell.x-symbol::before,
    .board.x-symbol .cell:not(.x-symbol):not(.o-symbol):hover::before {
        transform: rotate(45deg);
    }
    .cell.x-symbol::after,
    .board.x-symbol .cell:not(.x-symbol):not(.o-symbol):hover::after {
        transform: rotate(-45deg);
    }
    .cell.o-symbol::before,
    .cell.o-symbol::after,
    .board.o-symbol .cell:not(.x-symbol):not(.o-symbol):hover::before,
    .board.o-symbol .cell:not(.x-symbol):not(.o-symbol):hover::after {
        content: "";
        position: absolute;
        border-radius: 50%;
    }
    .cell.o-symbol::before,
    .board.o-symbol .cell:not(.x-symbol):not(.o-symbol):hover::before {
        width: var(--mark-size);
        height: var(--mark-size);
    }
    .cell.o-symbol::after,
    .board.o-symbol .cell:not(.x-symbol):not(.o-symbol):hover::after {
        width: calc(var(--mark-size) * 0.8);
        height: calc(var(--mark-size) * 0.8);
        background-color: white;
    }
    @media screen and (max-width: 800px) {
        .cell.x-symbol::before,
        .cell.x-symbol::after,
        .board.x-symbol .cell:not(.x-symbol):not(.o-symbol):hover::before,
        .board.x-symbol .cell:not(.x-symbol):not(.o-symbol):hover::after {
            width: calc(var(--mark-size-small) * 0.1);
            height: calc(var(--mark-size-small) * 1);
        }
        .cell.o-symbol::before,
        .board.o-symbol .cell:not(.x-symbol):not(.o-symbol):hover::before {
            width: var(--mark-size-small);
            height: var(--mark-size-small);
        }
        .cell.o-symbol::after,
        .board.o-symbol .cell:not(.x-symbol):not(.o-symbol):hover::after {
            width: calc(var(--mark-size-small) * 0.8);
            height: calc(var(--mark-size-small) * 0.8);
        }
    }
</style>
