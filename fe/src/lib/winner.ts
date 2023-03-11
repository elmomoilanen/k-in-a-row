import { Player } from "./players";

const MIN_OFFSET = 3;
const MIN_TOWIN = 3;

interface Cell {
    value: number;
}

interface Board {
    cells: Cell[];
    cellsOffset: number;
    cellsToWin: number;
}

export interface Result {
    hasWinner: boolean;
    winner?: Player;
    winCells?: number[];
}

function checkWinner(
    board: Board,
    player: Player,
    lastMoveCell: number,
    lastAllowedCell: number,
    stepSize: number
) {
    const match_indices = [];
    for (let i = lastMoveCell; i <= lastAllowedCell; i += stepSize) {
        if (board.cells[i].value === player) {
            match_indices.push(i);
        }
    }
    return match_indices.length === board.cellsToWin ? match_indices : undefined;
}

export function hasWinner(board: Board, lastMoveCell: number, player: Player) {
    const result: Result = { hasWinner: false };
    const cellsLen = board.cells.length;

    if (player === Player.Empty || lastMoveCell < 0 || lastMoveCell >= cellsLen) {
        return result;
    }
    if (board.cellsOffset < MIN_OFFSET || board.cellsToWin < MIN_TOWIN) {
        return result;
    }
    
    // Check winner in row
    let lastAllowedCell = lastMoveCell + board.cellsToWin - 1;
    if (lastAllowedCell < cellsLen) {
        const match_indices = [];
        for (let i = lastMoveCell; i <= lastAllowedCell; ++i) {
            if (board.cells[i].value === player) {
                match_indices.push(i);
            }
            if (i < lastAllowedCell && (i + 1) % board.cellsOffset === 0) {
                // Row boundary check
                break;
            }
        }
        if (match_indices.length === board.cellsToWin) {
            result["hasWinner"] = true;
            result["winner"] = player;
            result["winCells"] = match_indices;
            return result;
        }
    }
    // Check winner in column
    lastAllowedCell = lastMoveCell + (board.cellsToWin - 1) * board.cellsOffset;
    if (lastAllowedCell < cellsLen) {
        const winnerIndices = checkWinner(
            board,
            player,
            lastMoveCell,
            lastAllowedCell,
            board.cellsOffset
        );
        if (winnerIndices) {
            result["hasWinner"] = true;
            result["winner"] = player;
            result["winCells"] = winnerIndices;
            return result;
        }
    }
    // Check winner in diagonal
    lastAllowedCell = lastMoveCell + (board.cellsToWin - 1) * (board.cellsOffset + 1);
    if (lastAllowedCell < cellsLen) {
        const winnerIndices = checkWinner(
            board,
            player,
            lastMoveCell,
            lastAllowedCell,
            board.cellsOffset + 1
        );
        if (winnerIndices) {
            result["hasWinner"] = true;
            result["winner"] = player;
            result["winCells"] = winnerIndices;
            return result;
        }
    }
    // Check winner in antidiagonal
    lastAllowedCell = lastMoveCell + (board.cellsToWin - 1) * (board.cellsOffset - 1);
    if (lastAllowedCell < cellsLen) {
        const winnerIndices = checkWinner(
            board,
            player,
            lastMoveCell,
            lastAllowedCell,
            board.cellsOffset - 1
        );
        if (winnerIndices) {
            result["hasWinner"] = true;
            result["winner"] = player;
            result["winCells"] = winnerIndices;
            return result;
        }
    }
    return result;
}
