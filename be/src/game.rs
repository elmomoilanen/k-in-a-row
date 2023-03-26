use std::collections::HashSet;
use std::fmt::{self, Display, Formatter};

use rand::seq::SliceRandom;

use crate::conf::{BoardParams, BoardSize, GameInitError};
use crate::models::{Board, Level};

const P1_MARK: i8 = -1;
const BOT_MARK: i8 = 1;
const EMPTY_MARK: i8 = 0;

const WINNER_VALUE: i32 = 10000;
const ONE_TO_WIN_VALUE: i32 = 500;
const TWO_TO_WIN_VALUE: i32 = 100;
const OPPONENT_PENALTY_MULTIPLIER: i32 = 3;

pub struct Game {
    pub cells: Vec<i8>,
    pub p1_mark: i8,
    pub bot_mark: i8,
    pub empty_mark: i8,
    pub orig_p1_mark: i8,
    pub orig_bot_mark: i8,
    pub orig_empty_mark: i8,
    pub level: Level,
    pub board_size: BoardSize,
    pub max_depth: usize,
    cells_to_win: usize,
    cells_offset: usize,
}

impl Game {
    pub fn new(mut board: Board, level: Level) -> Result<Self, GameInitError> {
        let board_params = BoardParams::new(&board, level)?;

        Self::normalize_cell_values(&mut board);

        Ok(Game {
            cells: board.cells,
            p1_mark: P1_MARK,
            bot_mark: BOT_MARK,
            empty_mark: EMPTY_MARK,
            orig_p1_mark: board.p1_mark,
            orig_bot_mark: board.bot_mark,
            orig_empty_mark: board.empty_mark,
            level,
            board_size: board_params.size,
            max_depth: board_params.max_depth,
            cells_to_win: board_params.to_win,
            cells_offset: board_params.offset,
        })
    }

    pub fn empty_cell_count(&self) -> usize {
        Self::empty_cells(&self.cells, self.empty_mark)
    }

    pub fn empty_cell_indices(&mut self) -> Vec<usize> {
        let mut free_indices: Vec<usize> = self
            .cells
            .iter()
            .enumerate()
            .filter(|(_, &cell)| cell == self.empty_mark)
            .map(|(c_idx, _)| c_idx)
            .collect();

        free_indices.shuffle(&mut rand::thread_rng());

        match self.board_size {
            BoardSize::X33 | BoardSize::X44 => free_indices,
            _ => self.reorder_empty_cell_indices_by_value(free_indices),
        }
    }

    pub fn heuristic_game_value(&self, winner: i8, depth: i32) -> i32 {
        let depth_unzero = depth + 1;

        if winner != self.empty_mark {
            return winner as i32 * WINNER_VALUE * depth_unzero;
        }

        self.value_in_total() * depth_unzero
    }

    pub fn winner(&self) -> i8 {
        let winner_in_row = self.winner_in_row();
        if winner_in_row != self.empty_mark {
            return winner_in_row;
        }
        let winner_in_col = self.winner_in_col();
        if winner_in_col != self.empty_mark {
            return winner_in_col;
        }
        let winner_in_diag = self.winner_in_diag();
        if winner_in_diag != self.empty_mark {
            return winner_in_diag;
        }
        let winner_in_antidiag = self.winner_in_antidiag();
        if winner_in_antidiag != self.empty_mark {
            return winner_in_antidiag;
        }
        self.empty_mark
    }

    fn adjacent_cell_occupied(&self, index: usize) -> bool {
        let offset = self.cells_offset;
        let cells_count = self.cells.len();

        let adjacent_indices = if index == 0 {
            // First row three cases
            vec![index + 1, index + offset, index + offset + 1]
        } else if index == offset - 1 {
            vec![index - 1, index + (offset - 1), index + offset]
        } else if index < offset {
            vec![
                index - 1,
                index + 1,
                index + (offset - 1),
                index + offset,
                index + offset + 1,
            ]
        } else if index == cells_count - offset {
            // Last row three cases
            vec![index - offset, index - (offset - 1), index + 1]
        } else if index == cells_count - 1 {
            vec![index - (offset + 1), index - offset, index - 1]
        } else if index > cells_count - offset {
            vec![
                index - (offset + 1),
                index - offset,
                index - (offset - 1),
                index - 1,
                index + 1,
            ]
        } else if index % offset == 0 {
            // Leftmost column middle
            vec![
                index - offset,
                index - (offset - 1),
                index + 1,
                index + offset,
                index + offset + 1,
            ]
        } else if (index + 1) % offset == 0 {
            // Rightmost column middle
            vec![
                index - (offset + 1),
                index - offset,
                index - 1,
                index + (offset - 1),
                index + offset,
            ]
        } else {
            // Other
            vec![
                index - (offset + 1),
                index - offset,
                index - (offset - 1),
                index - 1,
                index + 1,
                index + (offset - 1),
                index + offset,
                index + offset + 1,
            ]
        };

        adjacent_indices
            .iter()
            .any(|&idx| self.cells[idx] != self.empty_mark)
    }

    fn empty_cells(slice: &[i8], empty_mark: i8) -> usize {
        slice.iter().filter(|&cell| *cell == empty_mark).count()
    }

    fn reorder_empty_cell_indices_by_value(&mut self, free_indices: Vec<usize>) -> Vec<usize> {
        let (free_indices_w_adj, _): (Vec<usize>, Vec<usize>) = free_indices
            .iter()
            .partition(|&index| self.adjacent_cell_occupied(*index));

        let mut value_map: Vec<(u32, i32)> = Vec::with_capacity(free_indices_w_adj.len());

        for &idx in free_indices_w_adj.iter() {
            self.cells[idx] = self.bot_mark;
            value_map.push((idx as u32, self.value_in_total()));
            self.cells[idx] = self.empty_mark;
        }
        value_map.sort_unstable_by_key(|&pair| -pair.1);

        value_map.iter().map(|&pair| pair.0 as usize).collect()
    }

    fn normalize_cell_values(board: &mut Board) {
        for cell in board.cells.iter_mut() {
            if *cell == board.p1_mark {
                *cell = P1_MARK;
            } else if *cell == board.bot_mark {
                *cell = BOT_MARK;
            } else {
                *cell = EMPTY_MARK;
            }
        }
    }

    fn winner_in_row(&self) -> i8 {
        let max_start_col = self.cells_offset - self.cells_to_win + 1;

        for start_col in 0..max_start_col {
            for (i, &row_first) in self
                .cells
                .iter()
                .enumerate()
                .skip(start_col)
                .step_by(self.cells_offset)
            {
                if row_first != self.empty_mark
                    && self
                        .cells
                        .iter()
                        .skip(i)
                        .take(self.cells_to_win)
                        .all(|&cell| cell == row_first)
                {
                    return row_first;
                }
            }
        }
        self.empty_mark
    }

    fn winner_in_col(&self) -> i8 {
        let max_start_row = self.cells_offset - self.cells_to_win + 1;

        for start_row in 0..max_start_row {
            for (i, &col_first) in self
                .cells
                .iter()
                .enumerate()
                .skip(start_row * self.cells_offset)
                .take(self.cells_offset)
            {
                if col_first != self.empty_mark
                    && self
                        .cells
                        .iter()
                        .skip(i)
                        .step_by(self.cells_offset)
                        .take(self.cells_to_win)
                        .all(|&cell| cell == col_first)
                {
                    return col_first;
                }
            }
        }
        self.empty_mark
    }

    fn diag_start_search_indices(&self, antidiag: bool) -> HashSet<usize> {
        let offset = self.cells_offset;
        let max_row = offset - self.cells_to_win + 1;
        let max_col = max_row;

        let start_search_indices: HashSet<usize> = if antidiag {
            let anti_offset = self.cells_to_win - 1;
            (0..max_row)
                .flat_map(|row| (row * offset + anti_offset..row * offset + anti_offset + max_col))
                .collect()
        } else {
            (0..max_row)
                .flat_map(|row| (row * offset..row * offset + max_col))
                .collect()
        };

        start_search_indices
    }

    fn winner_in_diag(&self) -> i8 {
        let offset = self.cells_offset;
        let max_row = offset - self.cells_to_win + 1;

        let start_search_indices = self.diag_start_search_indices(false);

        self.winner_general_diag(offset + 1, max_row, &start_search_indices)
    }

    fn winner_in_antidiag(&self) -> i8 {
        let offset = self.cells_offset;
        let max_row = offset - self.cells_to_win + 1;

        let start_search_indices = self.diag_start_search_indices(true);

        self.winner_general_diag(offset - 1, max_row, &start_search_indices)
    }

    fn winner_general_diag(
        &self,
        diag_offset: usize,
        max_row: usize,
        start_search_indices: &HashSet<usize>,
    ) -> i8 {
        for (i, &diag_first) in self
            .cells
            .iter()
            .enumerate()
            .take(max_row * self.cells_offset)
            .filter(|(i, _)| start_search_indices.contains(i))
        {
            if diag_first != self.empty_mark
                && self
                    .cells
                    .iter()
                    .skip(i)
                    .step_by(diag_offset)
                    .take(self.cells_to_win)
                    .all(|&cell| cell == diag_first)
            {
                return diag_first;
            }
        }
        self.empty_mark
    }

    fn value_in_total(&self) -> i32 {
        let mut value = 0;

        value += self.value_in_rows();
        value += self.value_in_cols();
        // Diagonal and antidiagonal evaluation
        value += self.value_in_diags(false);
        value += self.value_in_diags(true);

        value
    }

    fn value_in_rows(&self) -> i32 {
        let max_start_col = self.cells_offset - self.cells_to_win + 1;
        let mut value = 0;

        for start_col in 0..max_start_col {
            for row in 0..self.cells_offset {
                let start_cell = row * self.cells_offset + start_col;
                let row_slice = &self.cells[start_cell..start_cell + self.cells_to_win];

                let row_sum = row_slice.iter().sum::<i8>();
                let empty_cells = Self::empty_cells(row_slice, self.empty_mark);
                value +=
                    Self::compute_value_from_window_sum(row_sum, empty_cells, self.cells_to_win);
            }
        }
        value
    }

    fn value_in_cols(&self) -> i32 {
        let max_start_row = self.cells_offset - self.cells_to_win + 1;
        let mut value = 0;

        for start_row in 0..max_start_row {
            for col in 0..self.cells_offset {
                let start_cell = start_row * self.cells_offset + col;
                let col_slice: Vec<i8> = self.cells[start_cell..]
                    .iter()
                    .step_by(self.cells_offset)
                    .take(self.cells_to_win)
                    .copied()
                    .collect();

                let col_sum = col_slice.iter().sum::<i8>();
                let empty_cells = Self::empty_cells(&col_slice, self.empty_mark);
                value +=
                    Self::compute_value_from_window_sum(col_sum, empty_cells, self.cells_to_win);
            }
        }
        value
    }

    fn value_in_diags(&self, antidiag: bool) -> i32 {
        let diag_offset = if antidiag {
            self.cells_offset - 1
        } else {
            self.cells_offset + 1
        };

        let start_search_indices = self.diag_start_search_indices(antidiag);
        let mut value = 0;

        for &start_cell in start_search_indices.iter() {
            let diag_slice: Vec<i8> = self.cells[start_cell..]
                .iter()
                .step_by(diag_offset)
                .take(self.cells_to_win)
                .copied()
                .collect();

            let diag_sum = diag_slice.iter().sum::<i8>();
            let empty_cells = Self::empty_cells(&diag_slice, self.empty_mark);
            value += Self::compute_value_from_window_sum(diag_sum, empty_cells, self.cells_to_win);
        }
        value
    }

    fn compute_value_from_window_sum(
        window_sum: i8,
        window_empty_cells: usize,
        window: usize,
    ) -> i32 {
        let window_sum = window_sum as i32;
        let window_sum_abs = window_sum.unsigned_abs() as usize;
        let window_thres = window >> 1;

        // Following makes sense if `bot_mark` > 0 and `p1_mark` (opponent) < 0

        if window_sum_abs == window && window_sum > 0 {
            WINNER_VALUE
        } else if window_sum_abs == window - 1 && window_sum > 0 {
            window_sum * ONE_TO_WIN_VALUE
        } else if window_sum_abs == window - 1 && window_sum < 0 {
            window_sum * ONE_TO_WIN_VALUE * OPPONENT_PENALTY_MULTIPLIER
        } else if window_sum_abs >= window_thres && window_sum_abs + window_empty_cells == window {
            if window_sum > 0 {
                window_sum * TWO_TO_WIN_VALUE
            } else {
                window_sum * TWO_TO_WIN_VALUE * OPPONENT_PENALTY_MULTIPLIER
            }
        } else {
            0
        }
    }
}

impl Display for Game {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let show_cells_in_row = self.board_size as usize;

        for row in self.cells.chunks(show_cells_in_row) {
            let row_repr = row
                .iter()
                .map(|&val| {
                    if val == self.bot_mark {
                        'x'
                    } else if val == self.p1_mark {
                        'o'
                    } else {
                        '-'
                    }
                })
                .fold(String::new(), |acc, next| acc + &next.to_string() + " ");
            writeln!(f, "{row_repr}")?;
        }
        writeln!(f)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn init_game(cells: &[i8], p1_mark: i8, bot_mark: i8, empty_mark: i8) -> Game {
        let board = Board {
            cells: cells.to_vec(),
            p1_mark,
            bot_mark,
            empty_mark,
        };
        match Game::new(board, Level::Normal) {
            Ok(game) => game,
            Err(error_kind) => panic!("Game::new(): {:?}", error_kind),
        }
    }

    fn init_5x5_game_literal(cells: &[i8], p1_mark: i8, bot_mark: i8, empty_mark: i8) -> Game {
        // Use this e.g. to init an inconsistent game board
        Game {
            cells: cells.to_vec(),
            p1_mark,
            bot_mark,
            empty_mark,
            orig_p1_mark: p1_mark,
            orig_bot_mark: bot_mark,
            orig_empty_mark: empty_mark,
            level: Level::Normal,
            board_size: BoardSize::X55,
            max_depth: 7,
            cells_offset: BoardSize::X55 as usize,
            cells_to_win: 4,
        }
    }

    #[test]
    fn board_size_error() {
        let board = Board {
            cells: vec![0; 10],
            p1_mark: -1,
            bot_mark: 1,
            empty_mark: 0,
        };
        match Game::new(board, Level::Normal) {
            Ok(_) => panic!("Game::new() returned Ok."),
            Err(GameInitError::Size) => (),
            Err(error_kind) => panic!("Game::new() returned wrong error type {:?}", error_kind),
        }
    }

    #[test]
    fn board_mark_error() {
        let board = Board {
            cells: vec![0, 0, 0, 0, 0, 0, 0, 0, 2],
            p1_mark: 1,
            bot_mark: -1,
            empty_mark: 0,
        };
        match Game::new(board, Level::Normal) {
            Ok(_) => panic!("Game::new() returned Ok."),
            Err(GameInitError::Marks) => (),
            Err(error_kind) => panic!("Game::new() returned wrong error type {:?}", error_kind),
        }
    }

    #[test]
    fn board_inconsistent_error() {
        let cells_collections: [[i8; 9]; 2] =
            [[0, 0, 0, 0, 1, 0, 0, 0, 1], [0, 0, 0, 0, -1, 0, 0, 0, 0]];

        for cells in &cells_collections {
            let board = Board {
                cells: cells.to_vec(),
                p1_mark: 1,
                bot_mark: -1,
                empty_mark: 0,
            };
            match Game::new(board, Level::Normal) {
                Ok(_) => panic!("Game::new() returned Ok."),
                Err(GameInitError::Inconsistent) => (),
                Err(error_kind) => panic!("Game::new() returned wrong error type {:?}", error_kind),
            }
        }
    }

    #[test]
    fn init_new_game() {
        let (p1_mark, bot_mark, empty_mark) = (2, 11, 8);
        let board = Board {
            cells: vec![8, 8, 11, 11, 8, 8, 2, 8, 2],
            p1_mark,
            bot_mark,
            empty_mark,
        };
        let game = match Game::new(board, Level::Normal) {
            Ok(game) => game,
            Err(error_kind) => panic!("Game::new() returned error {:?}", error_kind),
        };

        let correct_cells = vec![0, 0, 1, 1, 0, 0, -1, 0, -1];

        // Test normalized marker values
        assert_eq!(game.p1_mark, P1_MARK);
        assert_eq!(game.bot_mark, BOT_MARK);
        assert_eq!(game.empty_mark, EMPTY_MARK);
        // Test original marker values
        assert_eq!(game.orig_p1_mark, p1_mark);
        assert_eq!(game.orig_bot_mark, bot_mark);
        assert_eq!(game.orig_empty_mark, empty_mark);

        assert_eq!(game.cells_offset, 3);
        assert_eq!(game.cells_to_win, 3);
        assert_eq!(game.cells, correct_cells);
    }

    #[test]
    fn empty_cell_indices_3x3() {
        // For board X33 all empty indices should always be returned
        let cells_collections: [[i8; 9]; 3] = [
            [0, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, -1, 0, 1, 0, 0, 0],
            [-1, 1, -1, 1, -1, 1, -1, 1, 0],
        ];
        let (p1_mark, bot_mark, empty_mark) = (1, -1, 0);

        for cells in &cells_collections {
            let true_empty_count = cells.iter().filter(|&cell| *cell == 0).count();
            let mut game = init_game(cells, p1_mark, bot_mark, empty_mark);

            let indices = game.empty_cell_indices();

            assert_eq!(indices.len(), true_empty_count);
            assert!(indices.iter().all(|&idx| idx < 9));
        }
    }

    #[test]
    fn empty_cell_indices_5x5() {
        // For larger boards (larger than X33) only empty cells
        // adjacent to an occupied cell should be returned
        let cells_collections: [[i8; 25]; 2] = [
            [
                1, 0, 0, 0, 0, 0, 0, 0, 0, -1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -1, -1,
            ],
            [
                -1, 1, -1, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 1, 1, 0, -1, 0, -1, -1,
            ],
        ];
        let (p1_mark, bot_mark, empty_mark) = (-1, 1, 0);
        let mut game = init_game(&cells_collections[0], p1_mark, bot_mark, empty_mark);

        let indices = game.empty_cell_indices();
        assert!(
            indices.len()
                < cells_collections[0]
                    .iter()
                    .filter(|&cell| *cell == 0)
                    .count()
        );
        // There are 15 adjacent empty cells for this board situation
        assert_eq!(indices.len(), 15);

        game = init_game(&cells_collections[1], p1_mark, bot_mark, empty_mark);

        let indices = game.empty_cell_indices();
        assert!(
            indices.len()
                < cells_collections[1]
                    .iter()
                    .filter(|&cell| *cell == 0)
                    .count()
        );
        // There are 14 adjacent empty cells this time
        assert_eq!(indices.len(), 14);
    }

    #[test]
    fn adjacent_cell_occupied_5x5() {
        let cells: [i8; 25] = [
            1, 1, 1, 0, 1, 1, 1, 1, 0, 0, 1, 1, 1, 1, 1, 1, 0, 0, 1, 1, 0, 1, 0, 1, 1,
        ];
        let (p1_mark, bot_mark, empty_mark) = (1, -1, 0);
        // Board (cells) is now inconsistent and cannot thus call Game::new directly
        let game = init_5x5_game_literal(&cells, p1_mark, bot_mark, empty_mark);

        // For an index test whether any of its nearby cells is occupied (has mark 1)
        let test_indices = [4, 6, 19, 21];
        let correct_adjacent_cell_occupied = [false, true, true, true];

        let it = test_indices
            .iter()
            .zip(correct_adjacent_cell_occupied.iter());

        for (&test_idx, &corr_result) in it {
            assert_eq!(game.adjacent_cell_occupied(test_idx), corr_result);
        }
    }

    fn compute_adjacent_cell_occupied_count_for_empty_cells(game: &Game) -> u32 {
        let mut count = 0;
        let it = game
            .cells
            .iter()
            .enumerate()
            .filter(|&(_, cell)| *cell == game.empty_mark);

        for (j, _) in it {
            if game.adjacent_cell_occupied(j) {
                count += 1;
            }
        }
        count
    }

    #[test]
    fn adjacent_cell_occupied_count_3x3() {
        let cells_collections: [[i8; 9]; 5] = [
            [0, 0, 0, 0, 1, 0, 0, 0, 0],
            [1, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 1, 0, 0, 0],
            [0, 0, 0, 1, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 1, 0],
        ];
        let (p1_mark, bot_mark, empty_mark) = (1, -1, 0);
        let correct_adjacent_count = [8, 3, 5, 5, 5];

        let it = cells_collections
            .iter()
            .enumerate()
            .zip(correct_adjacent_count);

        for ((j, cells), corr_adj_count) in it {
            let game = init_game(cells, p1_mark, bot_mark, empty_mark);
            let adj_res = compute_adjacent_cell_occupied_count_for_empty_cells(&game);
            assert_eq!(adj_res, corr_adj_count, "collection {j}");
        }
    }

    #[test]
    fn adjacent_cell_occupied_count_5x5() {
        let cells_collections: [[i8; 25]; 4] = [
            [
                1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            ],
            [
                0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            ],
            [
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            ],
            [
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0,
            ],
        ];
        let (p1_mark, bot_mark, empty_mark) = (1, -1, 0);
        let correct_adjacent_count = [3, 8, 8, 5];

        let it = cells_collections
            .iter()
            .enumerate()
            .zip(correct_adjacent_count);

        for ((j, cells), corr_adj_count) in it {
            let game = init_5x5_game_literal(cells, p1_mark, bot_mark, empty_mark);
            let adj_res = compute_adjacent_cell_occupied_count_for_empty_cells(&game);
            assert_eq!(adj_res, corr_adj_count, "collection {j}");
        }
    }

    #[test]
    fn row_winner_3x3() {
        let (p1_mark, bot_mark, empty_mark) = (1, -1, 0);

        let cells_collections: [[i8; 9]; 4] = [
            [-1, -1, 0, 1, 1, 1, 0, 0, 0],
            [1, 1, 1, -1, -1, 0, 0, 0, 0],
            [-1, 0, 0, 0, 0, -1, 1, 1, 1],
            [-1, -1, 0, -1, 0, 0, 1, 1, 1],
        ];

        for cells in &cells_collections {
            let game = init_game(cells, p1_mark, bot_mark, empty_mark);
            // Notice that `game.p1_mark` must be used on the right side (due to marker normalization)
            assert_eq!(game.winner_in_row(), game.p1_mark);
        }
    }

    #[test]
    fn col_winner_3x3() {
        let (p1_mark, bot_mark, empty_mark) = (1, -1, 0);

        let cells_collections: [[i8; 9]; 4] = [
            [-1, 1, 1, -1, 0, 0, -1, 0, 1],
            [1, -1, 1, 0, -1, 0, 0, -1, 1],
            [1, 1, -1, 1, 0, -1, 0, 0, -1],
            [1, -1, 1, 1, -1, -1, 0, -1, 1],
        ];

        for cells in &cells_collections {
            let game = init_game(cells, p1_mark, bot_mark, empty_mark);
            assert_eq!(game.winner_in_col(), game.bot_mark);
        }
    }

    #[test]
    fn diag_winner_3x3() {
        let (p1_mark, bot_mark, empty_mark) = (1, -1, 0);

        let cells_collections: [[i8; 9]; 4] = [
            [1, -1, -1, 0, 1, 0, 0, -1, 1],
            [1, 0, -1, 0, 1, -1, 0, -1, 1],
            // Next two have an antidiagonal winner
            [-1, 0, 1, 0, 1, 0, 1, -1, -1],
            [-1, -1, 1, -1, 1, 0, 1, 0, 0],
        ];

        for (j, cells) in cells_collections.iter().enumerate() {
            let game = init_game(cells, p1_mark, bot_mark, empty_mark);
            if j < 2 {
                assert_eq!(game.winner_in_diag(), game.p1_mark);
            } else {
                assert_eq!(game.winner_in_antidiag(), game.p1_mark);
            }
        }
    }

    #[test]
    fn row_winner_5x5() {
        let (p1_mark, bot_mark, empty_mark) = (1, -1, 0);

        let cells_collections: [[i8; 25]; 5] = [
            [
                1, 1, 1, 1, 0, -1, -1, -1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            ],
            [
                0, 1, 1, 1, 1, -1, -1, -1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            ],
            [
                -1, -1, -1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0,
            ],
            [
                1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -1, 0, -1, -1, 0, 0, 0, 0, 0,
            ],
            [
                1, 0, 1, 1, 0, -1, -1, -1, 1, 0, 0, -1, 0, 0, 0, 0, -1, -1, -1, 0, 0, 1, 1, 1, 1,
            ],
        ];

        for cells in &cells_collections {
            let game = init_game(cells, p1_mark, bot_mark, empty_mark);
            // Notice that `game.p1_mark` must be used on the right side (due to marker normalization)
            assert_eq!(game.winner_in_row(), game.p1_mark);
        }
    }

    #[test]
    fn col_winner_5x5() {
        let (p1_mark, bot_mark, empty_mark) = (1, -1, 0);

        let cells_collections: [[i8; 25]; 5] = [
            [
                1, -1, -1, -1, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            ],
            [
                -1, -1, -1, 0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0,
            ],
            [
                0, 0, -1, 0, 1, 0, 0, -1, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1, 0, -1, 0, -1, 0,
            ],
            [
                0, 0, -1, -1, -1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1,
            ],
            [
                -1, 0, 0, 0, 0, -1, 0, 1, 0, 0, -1, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1, 0, 0,
            ],
        ];

        for cells in &cells_collections {
            let game = init_game(cells, p1_mark, bot_mark, empty_mark);
            assert_eq!(game.winner_in_col(), game.p1_mark);
        }
    }

    #[test]
    fn diag_winner_5x5() {
        let (p1_mark, bot_mark, empty_mark) = (1, -1, 0);

        let cells_collections: [[i8; 25]; 4] = [
            [
                1, 0, -1, -1, -1, 0, 1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0,
            ],
            [
                0, 1, -1, -1, -1, 0, 0, 1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0,
            ],
            [
                0, -1, -1, -1, 0, 1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 1, 0,
            ],
            [
                -1, 0, -1, -1, 0, 0, 1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 1,
            ],
        ];

        for cells in &cells_collections {
            let game = init_game(cells, p1_mark, bot_mark, empty_mark);
            assert_eq!(game.winner_in_diag(), game.p1_mark);
        }
    }

    #[test]
    fn antidiag_winner_5x5() {
        let (p1_mark, bot_mark, empty_mark) = (1, -1, 0);

        let cells_collections: [[i8; 25]; 4] = [
            [
                -1, -1, -1, 1, -1, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            ],
            [
                0, 0, 0, 0, 1, 0, 0, 0, 1, -1, -1, -1, 1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0,
            ],
            [
                -1, 0, -1, -1, -1, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 0,
            ],
            [
                0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 1, -1, -1, -1,
            ],
        ];

        for cells in &cells_collections {
            let game = init_game(cells, p1_mark, bot_mark, empty_mark);
            assert_eq!(game.winner_in_antidiag(), game.p1_mark);
        }
    }

    #[test]
    fn value_in_rows() {
        let cells: [i8; 25] = [
            0, 0, 1, 1, 1, 1, -1, 1, 0, 0, 1, -1, -1, 1, 0, 0, 0, 0, 0, 0, 1, 0, 1, 0, 0,
        ];
        let (p1_mark, bot_mark, empty_mark) = (-1, 1, 0);
        // Board (cells) is now inconsistent and cannot thus call Game::new
        let game = init_5x5_game_literal(&cells, p1_mark, bot_mark, empty_mark);
        // First and last row bring value (cells' sum * value)
        // First row contribute for both cases, hence the extra "2 *" after + sign
        // Opponent's marks don't cause any penalty in this case
        let correct_value = 3 * ONE_TO_WIN_VALUE + 2 * 2 * TWO_TO_WIN_VALUE;

        assert_eq!(game.value_in_rows(), correct_value);
    }

    #[test]
    fn value_in_cols() {
        let cells: [i8; 25] = [
            0, 0, 0, 0, 1, 1, 1, 0, 0, 1, 0, 1, 1, 0, 0, 1, 0, 1, 0, 1, 0, 1, 1, 0, 0,
        ];
        let (p1_mark, bot_mark, empty_mark) = (-1, 1, 0);
        let game = init_5x5_game_literal(&cells, p1_mark, bot_mark, empty_mark);
        // 1st, 2nd, 3rd and 5th column bring value
        // 3 ONE_TO_WIN and 5 TWO_TO_WIN cases in total
        let correct_value = 3 * 3 * ONE_TO_WIN_VALUE + 5 * 2 * TWO_TO_WIN_VALUE;

        assert_eq!(game.value_in_cols(), correct_value);
    }

    #[test]
    fn value_in_diagonals() {
        let cells: [i8; 25] = [
            0, 1, 0, 0, 0, 1, 1, 0, 0, 0, 0, 1, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1, 1,
        ];
        let (p1_mark, bot_mark, empty_mark) = (-1, 1, 0);
        let game = init_5x5_game_literal(&cells, p1_mark, bot_mark, empty_mark);
        // 2 ONE_TO_WIN and 2 TWO_TO_WIN cases in total
        let correct_value = 2 * 3 * ONE_TO_WIN_VALUE + 2 * 2 * TWO_TO_WIN_VALUE;

        assert_eq!(game.value_in_diags(false), correct_value);
    }

    #[test]
    fn value_in_antidiagonals() {
        let cells: [i8; 25] = [
            0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 0, 1, 1, 1, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0,
        ];
        let (p1_mark, bot_mark, empty_mark) = (-1, 1, 0);
        let game = init_5x5_game_literal(&cells, p1_mark, bot_mark, empty_mark);
        // 1 ONE_TO_WIN and 3 TWO_TO_WIN cases in total
        let correct_value = 3 * ONE_TO_WIN_VALUE + 3 * 2 * TWO_TO_WIN_VALUE;

        assert_eq!(game.value_in_diags(true), correct_value);
    }

    #[test]
    fn value_penalty_opposite_player() {
        let cells: [i8; 25] = [
            0, -1, -1, -1, 0, 0, 0, 0, 0, 0, -1, 1, -1, -1, 0, 0, 0, 0, 0, 0, -1, 0, -1, 0, 1,
        ];
        let (p1_mark, bot_mark, empty_mark) = (1, -1, 0);
        let game = init_5x5_game_literal(&cells, p1_mark, bot_mark, empty_mark);

        let correct_value_first_row = 2 * -3 * ONE_TO_WIN_VALUE * OPPONENT_PENALTY_MULTIPLIER;
        let correct_value_last_row = 1 * -2 * TWO_TO_WIN_VALUE * OPPONENT_PENALTY_MULTIPLIER;
        let correct_value = correct_value_first_row + correct_value_last_row;

        assert_eq!(game.value_in_rows(), correct_value);
    }
}
