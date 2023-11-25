type GameType =
    | "x33"
    | "x44"
    | "x55"
    | "x66"
    | "x77"
    | "x88"
    | "x99"
    | "x1010"
    | "x1111"
    | "x1212"
    | "x1313"
    | "x1414"
    | "x1515";
type LevelName = "easy" | "normal";

export interface Game {
    readonly cellsTotal: number;
    readonly cellsToWinMin: number;
    readonly cellsToWinMax: number;
    readonly boardSize: string;
    readonly gameKey: string;
    cellsToWin: number;
}

export interface GameLevel {
    readonly levelName: string;
}

export const Games: Record<GameType, Game> = {
    x33: {
        cellsTotal: 9,
        cellsToWinMin: 3,
        cellsToWinMax: 3,
        boardSize: "3x3",
        gameKey: "x33",
        cellsToWin: 0
    },
    x44: {
        cellsTotal: 16,
        cellsToWinMin: 4,
        cellsToWinMax: 4,
        boardSize: "4x4",
        gameKey: "x44",
        cellsToWin: 0
    },
    x55: {
        cellsTotal: 25,
        cellsToWinMin: 4,
        cellsToWinMax: 5,
        boardSize: "5x5",
        gameKey: "x55",
        cellsToWin: 0
    },
    x66: {
        cellsTotal: 36,
        cellsToWinMin: 4,
        cellsToWinMax: 6,
        boardSize: "6x6",
        gameKey: "x66",
        cellsToWin: 0
    },
    x77: {
        cellsTotal: 49,
        cellsToWinMin: 4,
        cellsToWinMax: 7,
        boardSize: "7x7",
        gameKey: "x77",
        cellsToWin: 0
    },
    x88: {
        cellsTotal: 64,
        cellsToWinMin: 4,
        cellsToWinMax: 8,
        boardSize: "8x8",
        gameKey: "x88",
        cellsToWin: 0
    },
    x99: {
        cellsTotal: 81,
        cellsToWinMin: 4,
        cellsToWinMax: 9,
        boardSize: "9x9",
        gameKey: "x99",
        cellsToWin: 0
    },
    x1010: {
        cellsTotal: 100,
        cellsToWinMin: 5,
        cellsToWinMax: 10,
        boardSize: "10x10",
        gameKey: "x1010",
        cellsToWin: 0
    },
    x1111: {
        cellsTotal: 121,
        cellsToWinMin: 5,
        cellsToWinMax: 10,
        boardSize: "11x11",
        gameKey: "x1111",
        cellsToWin: 0
    },
    x1212: {
        cellsTotal: 144,
        cellsToWinMin: 5,
        cellsToWinMax: 10,
        boardSize: "12x12",
        gameKey: "x1212",
        cellsToWin: 0
    },
    x1313: {
        cellsTotal: 169,
        cellsToWinMin: 5,
        cellsToWinMax: 10,
        boardSize: "13x13",
        gameKey: "x1313",
        cellsToWin: 0
    },
    x1414: {
        cellsTotal: 196,
        cellsToWinMin: 5,
        cellsToWinMax: 10,
        boardSize: "14x14",
        gameKey: "x1414",
        cellsToWin: 0
    },
    x1515: {
        cellsTotal: 225,
        cellsToWinMin: 5,
        cellsToWinMax: 10,
        boardSize: "15x15",
        gameKey: "x1515",
        cellsToWin: 0
    }
};

export const GameLevels: Record<LevelName, GameLevel> = {
    easy: { levelName: "Easy" },
    normal: { levelName: "Normal" }
};

export type MaybeGame = Game | undefined;
export type MaybeGameLevel = GameLevel | undefined;
