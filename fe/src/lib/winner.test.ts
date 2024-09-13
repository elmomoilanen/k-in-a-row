import { test, expect } from "vitest";
import { Player } from "./players";
import { hasWinner } from "./winner";

interface Board {
    cells: { value: number }[];
    cellsOffset: number;
    cellsToWin: number;
}

function createBoard(cells: number[], cellsOffset: number, cellsToWin: number): Board {
    return {
        cells: cells.map((value) => ({ value })),
        cellsOffset,
        cellsToWin
    };
}

test("detect a winner in the first row of a 3x3 board", () => {
    const board = createBoard(
        [
            Player.P1,
            Player.P1,
            Player.P1,
            Player.Bot,
            Player.Bot,
            Player.Empty,
            Player.Empty,
            Player.Empty,
            Player.Empty
        ],
        3,
        3
    );
    expect(board.cells.length).toBe(9);
    const result = hasWinner(board, 2, Player.P1);
    expect(result.hasWinner).toBe(true);
    expect(result.winner).toBe(Player.P1);
    expect(result.winCells).toEqual([0, 1, 2]);
});

test("detect a winner in the second column of a 3x3 board", () => {
    const board = createBoard(
        [
            Player.Empty,
            Player.P1,
            Player.Empty,
            Player.Empty,
            Player.P1,
            Player.Empty,
            Player.Empty,
            Player.P1,
            Player.Empty
        ],
        3,
        3
    );
    expect(board.cells.length).toBe(9);
    const result = hasWinner(board, 7, Player.P1);
    expect(result.hasWinner).toBe(true);
    expect(result.winner).toBe(Player.P1);
    expect(result.winCells).toEqual([1, 4, 7]);
});

test("detect a winner in the diagonal of a 3x3 board", () => {
    const board = createBoard(
        [
            Player.P1,
            Player.Empty,
            Player.Empty,
            Player.Empty,
            Player.P1,
            Player.Empty,
            Player.Empty,
            Player.Empty,
            Player.P1
        ],
        3,
        3
    );
    expect(board.cells.length).toBe(9);
    const result = hasWinner(board, 4, Player.P1);
    expect(result.hasWinner).toBe(true);
    expect(result.winner).toBe(Player.P1);
    expect(result.winCells).toEqual([0, 4, 8]);
});

test("detect a winner in the antidiagonal of a 3x3 board", () => {
    const board = createBoard(
        [
            Player.Empty,
            Player.Empty,
            Player.P1,
            Player.Empty,
            Player.P1,
            Player.Empty,
            Player.P1,
            Player.Empty,
            Player.Empty
        ],
        3,
        3
    );
    expect(board.cells.length).toBe(9);
    const result = hasWinner(board, 6, Player.P1);
    expect(result.hasWinner).toBe(true);
    expect(result.winner).toBe(Player.P1);
    expect(result.winCells).toEqual([2, 4, 6]);
});

test("detect a winner in the 3rd row of a 6x6 5-in-a-row game", () => {
    const board = createBoard(
        [
            Player.Bot,
            Player.P1,
            Player.Bot,
            Player.Bot,
            Player.Bot,
            Player.P1,
            Player.P1,
            Player.P1,
            Player.P1,
            Player.P1,
            Player.Empty,
            Player.Empty,
            Player.P1,
            Player.Bot,
            Player.Bot,
            Player.Bot,
            Player.Bot,
            Player.Bot,
            Player.Empty,
            Player.Empty,
            Player.Empty,
            Player.Empty,
            Player.Empty,
            Player.Empty,
            Player.P1,
            Player.Empty,
            Player.P1,
            Player.P1,
            Player.Empty,
            Player.Empty,
            Player.Empty,
            Player.Empty,
            Player.Empty,
            Player.Empty,
            Player.Empty,
            Player.Empty
        ],
        6,
        5
    );
    expect(board.cells.length).toBe(36);
    const result = hasWinner(board, 17, Player.Bot);
    expect(result.hasWinner).toBe(true);
    expect(result.winner).toBe(Player.Bot);
    expect(result.winCells).toEqual([13, 14, 15, 16, 17]);
});

test("detect a winner in the 5rd column of a 6x6 5-in-a-row game", () => {
    const board = createBoard(
        [
            Player.Bot,
            Player.P1,
            Player.Bot,
            Player.Empty,
            Player.Empty,
            Player.P1,
            Player.P1,
            Player.P1,
            Player.P1,
            Player.P1,
            Player.Bot,
            Player.Empty,
            Player.P1,
            Player.Bot,
            Player.P1,
            Player.Empty,
            Player.Bot,
            Player.P1,
            Player.Empty,
            Player.P1,
            Player.Empty,
            Player.Empty,
            Player.Bot,
            Player.Empty,
            Player.Empty,
            Player.P1,
            Player.Empty,
            Player.Empty,
            Player.Bot,
            Player.Empty,
            Player.Empty,
            Player.Bot,
            Player.Bot,
            Player.Empty,
            Player.Bot,
            Player.Empty
        ],
        6,
        5
    );
    expect(board.cells.length).toBe(36);
    const result = hasWinner(board, 34, Player.Bot);
    expect(result.hasWinner).toBe(true);
    expect(result.winner).toBe(Player.Bot);
    expect(result.winCells).toEqual([10, 16, 22, 28, 34]);
});

test("detect a winner in the diagonal of a 6x6 board 5-in-a-row game", () => {
    const board = createBoard(
        [
            Player.Empty,
            Player.Empty,
            Player.Empty,
            Player.Empty,
            Player.Empty,
            Player.Empty,
            Player.Empty,
            Player.Bot,
            Player.Empty,
            Player.Empty,
            Player.Empty,
            Player.Empty,
            Player.Empty,
            Player.Empty,
            Player.Bot,
            Player.Empty,
            Player.Empty,
            Player.Empty,
            Player.Empty,
            Player.Empty,
            Player.Empty,
            Player.Bot,
            Player.Empty,
            Player.Empty,
            Player.Empty,
            Player.Empty,
            Player.Empty,
            Player.Empty,
            Player.Bot,
            Player.Empty,
            Player.Empty,
            Player.Empty,
            Player.Empty,
            Player.Empty,
            Player.Empty,
            Player.Bot
        ],
        6,
        5
    );
    expect(board.cells.length).toBe(36);
    const result = hasWinner(board, 7, Player.Bot);
    expect(result.hasWinner).toBe(true);
    expect(result.winner).toBe(Player.Bot);
    expect(result.winCells).toEqual([7, 14, 21, 28, 35]);
});

test("detect a winner in the antidiagonal of a 6x6 board 5-in-a-row game", () => {
    const board = createBoard(
        [
            Player.Empty,
            Player.Empty,
            Player.Empty,
            Player.Empty,
            Player.Empty,
            Player.Bot,
            Player.Empty,
            Player.Empty,
            Player.Empty,
            Player.Empty,
            Player.Bot,
            Player.Empty,
            Player.Empty,
            Player.Empty,
            Player.Empty,
            Player.Bot,
            Player.Empty,
            Player.Empty,
            Player.Empty,
            Player.Empty,
            Player.Bot,
            Player.Empty,
            Player.Empty,
            Player.Empty,
            Player.Empty,
            Player.Bot,
            Player.Empty,
            Player.Empty,
            Player.Empty,
            Player.Empty,
            Player.Empty,
            Player.Empty,
            Player.Empty,
            Player.Empty,
            Player.Empty,
            Player.Empty
        ],
        6,
        5
    );
    expect(board.cells.length).toBe(36);
    const result = hasWinner(board, 15, Player.Bot);
    expect(result.hasWinner).toBe(true);
    expect(result.winner).toBe(Player.Bot);
    expect(result.winCells).toEqual([5, 10, 15, 20, 25]);
});
