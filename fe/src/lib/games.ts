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
    readonly cellsToWin: number;
    readonly boardSize: string;
    readonly gameKey: string;
}

export interface GameLevel {
    readonly levelName: string;
}

export const Games: Record<GameType, Game> = {
    x33: { cellsTotal: 9, cellsToWin: 3, boardSize: "3x3", gameKey: "x33" },
    x44: { cellsTotal: 16, cellsToWin: 4, boardSize: "4x4", gameKey: "x44" },
    x55: { cellsTotal: 25, cellsToWin: 4, boardSize: "5x5", gameKey: "x55" },
    x66: { cellsTotal: 36, cellsToWin: 5, boardSize: "6x6", gameKey: "x66" },
    x77: { cellsTotal: 49, cellsToWin: 5, boardSize: "7x7", gameKey: "x77" },
    x88: { cellsTotal: 64, cellsToWin: 5, boardSize: "8x8", gameKey: "x88" },
    x99: { cellsTotal: 81, cellsToWin: 6, boardSize: "9x9", gameKey: "x99" },
    x1010: {
        cellsTotal: 100,
        cellsToWin: 5,
        boardSize: "10x10",
        gameKey: "x1010"
    },
    x1111: {
        cellsTotal: 121,
        cellsToWin: 5,
        boardSize: "11x11",
        gameKey: "x1111"
    },
    x1212: {
        cellsTotal: 144,
        cellsToWin: 5,
        boardSize: "12x12",
        gameKey: "x1212"
    },
    x1313: {
        cellsTotal: 169,
        cellsToWin: 6,
        boardSize: "13x13",
        gameKey: "x1313"
    },
    x1414: {
        cellsTotal: 196,
        cellsToWin: 7,
        boardSize: "14x14",
        gameKey: "x1414"
    },
    x1515: {
        cellsTotal: 225,
        cellsToWin: 5,
        boardSize: "15x15",
        gameKey: "x1515"
    }
};

export const GameLevels: Record<LevelName, GameLevel> = {
    easy: { levelName: "Easy" },
    normal: { levelName: "Normal" }
};

export type MaybeGame = Game | undefined;
export type MaybeGameLevel = GameLevel | undefined;
