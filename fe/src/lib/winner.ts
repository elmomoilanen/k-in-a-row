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
    startCell: number,
    lastAllowedCell: number,
    stepSize: number
) {
    const match_indices = [];
    for (let i = startCell; i <= lastAllowedCell; i += stepSize) {
        if (board.cells[i].value === player) {
            match_indices.push(i);
        }
    }
    return match_indices.length === board.cellsToWin ? match_indices : undefined;
}

function checkDiagonal(board: Board, lastMoveCell: number, player: Player) {
    let doDiagonalSearch = true;
    let firstStartCheckCell: number;

    if (lastMoveCell < board.cellsOffset || lastMoveCell % board.cellsOffset === 0) {
        // From these cells cannot move further leftward
        firstStartCheckCell = lastMoveCell;
    } else {
        // Multiplier depends on whether `lastMoveCell` is located below (min is left) or above the diagonal (right)
        const multiplier = Math.min(
            lastMoveCell % board.cellsOffset,
            Math.floor(lastMoveCell / board.cellsOffset)
        );
        firstStartCheckCell = lastMoveCell - multiplier * (board.cellsOffset + 1);
    }
    if (
        firstStartCheckCell < board.cellsOffset &&
        firstStartCheckCell > board.cellsOffset - board.cellsToWin
    ) {
        // From this cell there cannot be enough cells in a (diagonal) row to win
        doDiagonalSearch = false;
    }
    if (
        firstStartCheckCell % board.cellsOffset === 0 &&
        firstStartCheckCell > board.cells.length - board.cellsOffset * board.cellsToWin
    ) {
        doDiagonalSearch = false;
    }

    let lastCellToCheck = firstStartCheckCell + (board.cellsOffset + 1) * (board.cellsToWin - 1);

    while (doDiagonalSearch && lastCellToCheck < board.cells.length) {
        const winnerIndices = checkWinner(
            board,
            player,
            firstStartCheckCell,
            lastCellToCheck,
            board.cellsOffset + 1
        );
        if (winnerIndices) {
            return winnerIndices;
        }
        firstStartCheckCell += board.cellsOffset + 1;
        lastCellToCheck = firstStartCheckCell + (board.cellsOffset + 1) * (board.cellsToWin - 1);
    }
    return undefined;
}

function checkAntiDiagonal(board: Board, lastMoveCell: number, player: Player) {
    let doDiagonalSearch = true;
    let firstStartCheckCell: number;

    if (lastMoveCell < board.cellsOffset || (lastMoveCell + 1) % board.cellsOffset === 0) {
        // From these cells cannot move further rightward
        firstStartCheckCell = lastMoveCell;
    } else {
        // Multiplier depends on whether `lastMoveCell` is located below (min is left) or above the diagonal (right)
        const multiplier = Math.min(
            board.cellsOffset - (lastMoveCell % board.cellsOffset) - 1,
            Math.floor(lastMoveCell / board.cellsOffset)
        );
        firstStartCheckCell = lastMoveCell - multiplier * (board.cellsOffset - 1);
    }
    if (firstStartCheckCell < board.cellsToWin - 1) {
        // From this cell there cannot be enough cells in a (antidiagonal) row to win
        doDiagonalSearch = false;
    }
    if (
        (firstStartCheckCell + 1) % board.cellsOffset === 0 &&
        firstStartCheckCell >= board.cells.length - board.cellsOffset * (board.cellsToWin - 1)
    ) {
        doDiagonalSearch = false;
    }

    let lastCellToCheck = firstStartCheckCell + (board.cellsOffset - 1) * (board.cellsToWin - 1);

    while (doDiagonalSearch && lastCellToCheck < board.cells.length) {
        const winnerIndices = checkWinner(
            board,
            player,
            firstStartCheckCell,
            lastCellToCheck,
            board.cellsOffset - 1
        );
        if (winnerIndices) {
            return winnerIndices;
        }
        firstStartCheckCell += board.cellsOffset - 1;
        lastCellToCheck = firstStartCheckCell + (board.cellsOffset - 1) * (board.cellsToWin - 1);
    }
    return undefined;
}

export function hasWinner(board: Board, lastMoveCell: number, player: Player) {
    const result: Result = { hasWinner: false };
    const cellsLen = board.cells.length;

    if (player === Player.Empty || lastMoveCell < 0 || lastMoveCell >= cellsLen) {
        return result;
    }
    // Do few checks just to make sure that assumptions hold below
    if (
        board.cellsOffset < MIN_OFFSET ||
        board.cellsToWin < MIN_TOWIN ||
        board.cellsOffset < board.cellsToWin
    ) {
        return result;
    }

    // Check possible winner in the row containing `lastMoveCell`
    let firstStartCheckCell = lastMoveCell - (lastMoveCell % board.cellsOffset);
    let lastStartCheckCell = firstStartCheckCell + board.cellsOffset - board.cellsToWin;

    for (let i = firstStartCheckCell; i <= lastStartCheckCell; ++i) {
        const lastAllowedCell = i + board.cellsToWin - 1;
        if (lastAllowedCell < cellsLen) {
            const winnerIndices = checkWinner(board, player, i, lastAllowedCell, 1);
            if (winnerIndices) {
                Object.assign(result, { hasWinner: true, winner: player, winCells: winnerIndices });
                return result;
            }
        }
    }

    // Check possible winner in the column containing `lastMoveCell`
    firstStartCheckCell = lastMoveCell % board.cellsOffset;
    lastStartCheckCell =
        firstStartCheckCell + (board.cellsOffset - board.cellsToWin) * board.cellsOffset;

    for (let i = firstStartCheckCell; i <= lastStartCheckCell; i += board.cellsOffset) {
        const lastAllowedCell = i + (board.cellsToWin - 1) * board.cellsOffset;
        if (lastAllowedCell < cellsLen) {
            const winnerIndices = checkWinner(board, player, i, lastAllowedCell, board.cellsOffset);
            if (winnerIndices) {
                Object.assign(result, { hasWinner: true, winner: player, winCells: winnerIndices });
                return result;
            }
        }
    }

    // Check possible winner in the diagonal containing `lastMoveCell`
    const winnerIndicesDiag = checkDiagonal(board, lastMoveCell, player);
    if (winnerIndicesDiag) {
        Object.assign(result, { hasWinner: true, winner: player, winCells: winnerIndicesDiag });
        return result;
    }

    // Check possible winner in the antidiagonal containing `lastMoveCell`
    const winnerIndicesAntiDiag = checkAntiDiagonal(board, lastMoveCell, player);
    if (winnerIndicesAntiDiag) {
        Object.assign(result, {
            hasWinner: true,
            winner: player,
            winCells: winnerIndicesAntiDiag
        });
        return result;
    }

    return result;
}
