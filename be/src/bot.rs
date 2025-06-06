use rand::{seq::SliceRandom, Rng};
use std::cmp;

use crate::{
    conf::BoardSize,
    first_move::FirstMove,
    game::Game,
    models::{BotMove, Level},
};

const EASY_LEVEL_RANDOM_MOVE_THRESHOLD: f32 = 0.67;

/// Type to represent the computer player, aka bot.
///
/// Use method `next_move` to play one round of a game.
///
/// # Examples
///
/// Play the first move of a 3x3 3-in-a-row game in normal mode
///
/// ```
/// use be::{Board, Bot, Game, Level};
///
/// let board = Board {
///     cells: vec![0, 0, 0, 0, 0, 0, 0, 0, 0],
///     cells_to_win: 3,
///     p1_mark: 1,
///     bot_mark: -1,
///     empty_mark: 0,
/// };
///
/// let game = match Game::new(board, Level::Normal) {
///     Ok(game) => game,
///     Err(error) => panic!("Game::new(): {:?}", error),
/// };
///
/// let bot_next_move = Bot::next_move(game);
///
/// // Check that the move is valid within the 3x3 board
/// assert!((0..9).contains(&bot_next_move.next));
/// assert!(bot_next_move.next_is_valid);
/// assert!(!bot_next_move.game_over);
/// ```
pub struct Bot;

impl Bot {
    pub fn next_move(mut game: Game) -> BotMove {
        let cells_count = game.cells.len();
        let empty_cells = game.empty_cell_count();

        if empty_cells == cells_count {
            return Self::play_game_first_move(game, cells_count);
        }

        if empty_cells == cells_count - 1 {
            if let Some(bot_move) = Self::play_bot_first_move_if_defined(&game) {
                return bot_move;
            }
        }

        if let Level::Easy = game.level {
            if rand::random::<f32>() > EASY_LEVEL_RANDOM_MOVE_THRESHOLD {
                return Self::play_bot_random_move(game);
            }
        }

        let init_depth = cmp::min(empty_cells, game.max_depth);
        let first_player = game.bot_mark;

        let (_, best_move) = Self::minimax(
            &mut game,
            first_player,
            init_depth as i32,
            i32::MIN,
            i32::MAX,
            true,
        );

        Self::complete_bot_move(game, best_move)
    }

    fn play_game_first_move(game: Game, cells_count: usize) -> BotMove {
        BotMove {
            next: rand::thread_rng().gen_range(0..cells_count as u8),
            next_is_valid: true,
            game_over: false,
            winner: game.orig_empty_mark,
        }
    }

    fn play_bot_first_move_if_defined(game: &Game) -> Option<BotMove> {
        let p1_mark_pos = game.cells.iter().position(|&cell| cell == game.p1_mark);

        let bot_next_pos = match (p1_mark_pos, game.board_size) {
            (Some(p1_idx), BoardSize::X55) => FirstMove::find_bot_first_move_5x5(p1_idx),
            _ => return None,
        };

        Some(BotMove {
            next: bot_next_pos,
            next_is_valid: true,
            game_over: false,
            winner: game.orig_empty_mark,
        })
    }

    fn play_bot_random_move(mut game: Game) -> BotMove {
        let winner = game.winner();

        if winner != game.empty_mark || game.empty_cell_count() == 0 {
            return Self::complete_bot_move(game, None);
        }

        let mut empty_cells = game.empty_cell_indices();
        empty_cells.shuffle(&mut rand::thread_rng());

        match empty_cells.first() {
            Some(&cell_idx) => Self::complete_bot_move(game, Some(cell_idx)),
            None => Self::complete_bot_move(game, None),
        }
    }

    fn renormalize_winner_marker(game: &Game, winner: i8) -> i8 {
        if winner == game.empty_mark {
            game.orig_empty_mark
        } else if winner == game.p1_mark {
            game.orig_p1_mark
        } else {
            game.orig_bot_mark
        }
    }

    fn complete_bot_move(mut game: Game, best_next_move: Option<usize>) -> BotMove {
        match best_next_move {
            Some(best_move) => {
                game.cells[best_move] = game.bot_mark;

                let winner = game.winner();
                let game_over = winner != game.empty_mark || game.empty_cell_count() == 0;
                let winner_orig = Self::renormalize_winner_marker(&game, winner);

                BotMove {
                    next: best_move as u8,
                    next_is_valid: true,
                    game_over,
                    winner: winner_orig,
                }
            }
            None => BotMove {
                next: u8::MAX,
                next_is_valid: false,
                game_over: true,
                winner: Self::renormalize_winner_marker(&game, game.winner()),
            },
        }
    }

    fn minimax(
        game: &mut Game,
        player: i8,
        depth: i32,
        mut alpha: i32,
        mut beta: i32,
        maximize: bool,
    ) -> (i32, Option<usize>) {
        let winner = game.winner();

        if winner != game.empty_mark || depth <= 0 || game.empty_cell_count() == 0 {
            return (game.heuristic_game_value(winner, depth), None);
        }

        let mut best_entry = None;
        let mut best_value = if maximize { i32::MIN } else { i32::MAX };

        let empty_cells = game.empty_cell_indices();

        for &empty_cell in empty_cells.iter() {
            game.cells[empty_cell] = player;

            let next_player = if player == game.bot_mark {
                game.p1_mark
            } else {
                game.bot_mark
            };

            if maximize {
                let (value, _) = Self::minimax(game, next_player, depth - 1, alpha, beta, false);
                if value > best_value {
                    best_value = value;
                    best_entry = Some(empty_cell);
                }
                game.cells[empty_cell] = game.empty_mark;

                alpha = cmp::max(alpha, best_value);
                if beta <= alpha {
                    break;
                }
            } else {
                let (value, _) = Self::minimax(game, next_player, depth - 1, alpha, beta, true);
                if value < best_value {
                    best_value = value;
                    best_entry = Some(empty_cell);
                }
                game.cells[empty_cell] = game.empty_mark;

                beta = cmp::min(beta, best_value);
                if beta <= alpha {
                    break;
                }
            }
        }

        (best_value, best_entry)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::conf::*;
    use crate::models::{Board, Level};
    use std::{cmp::Ordering, mem};

    fn init_game(
        cells: &[i8],
        p1_mark: i8,
        bot_mark: i8,
        empty_mark: i8,
        cells_to_win: u8,
    ) -> Game {
        let board = Board {
            cells: cells.to_vec(),
            cells_to_win,
            p1_mark,
            bot_mark,
            empty_mark,
        };
        match Game::new(board, Level::Normal) {
            Ok(game) => game,
            Err(error_kind) => panic!("Game::new(): {:?}", error_kind),
        }
    }

    fn run_minimax_for_bot(mut game: Game) -> Option<usize> {
        let empty_cells = game.empty_cell_count();
        let first_player = game.bot_mark;

        let init_depth = cmp::min(empty_cells, game.max_depth);

        let (_, best_move) = Bot::minimax(
            &mut game,
            first_player,
            init_depth as i32,
            i32::MIN,
            i32::MAX,
            true,
        );

        best_move
    }

    #[test]
    fn bot_make_win_move_3x3() {
        let (p1_mark, bot_mark, empty_mark) = (-1, 1, 0);
        let cells_to_win = X33_CELLS_TO_WIN_MIN;

        let cells_collections: [[i8; 9]; 12] = [
            [-1, 1, -1, 0, 1, 0, 1, 0, -1],
            [-1, 1, 0, 0, 1, 0, -1, 0, 0],
            [1, 1, 0, 0, -1, 0, 0, 0, -1],
            [0, 0, 0, -1, -1, 0, 0, 1, 1],
            [1, 0, -1, -1, 0, 0, 0, 0, 1],
            [1, -1, -1, -1, 1, 1, -1, 0, 0],
            [-1, 0, -1, -1, 1, 0, 1, 1, -1],
            [-1, 0, 1, 1, 1, -1, -1, 1, -1],
            [0, 1, 0, -1, 0, 0, 0, 1, -1],
            [0, 1, 1, -1, -1, 0, 1, -1, 0],
            [1, -1, 0, 0, -1, 0, 1, 0, 0],
            [0, 0, -1, 1, 1, 0, 0, -1, 0],
        ];
        let correct_win_moves: [usize; 12] = [7, 7, 2, 6, 4, 8, 1, 1, 4, 0, 3, 5];

        let it = cells_collections.iter().zip(correct_win_moves.iter());

        for (i, (cells, &correct_move)) in it.enumerate() {
            let game = init_game(cells, p1_mark, bot_mark, empty_mark, cells_to_win);

            match run_minimax_for_bot(game) {
                Some(best_move) => assert_eq!(best_move, correct_move, "collection {}", i + 1),
                _ => panic!("Bot::minimax (collection {i}) returned None"),
            }
        }
    }

    #[test]
    fn bot_prevent_win_move_3x3() {
        let (p1_mark, bot_mark, empty_mark) = (-1, 1, 0);
        let cells_to_win = X33_CELLS_TO_WIN_MIN;

        let cells_collections: [[i8; 9]; 12] = [
            [1, -1, -1, -1, -1, 1, 1, 0, 0],
            [1, -1, 0, 0, -1, 1, 0, 0, 0],
            [0, 1, -1, 0, -1, 1, 1, 0, -1],
            [0, -1, -1, -1, 1, 1, 0, 1, 0],
            [0, -1, -1, -1, 1, 1, -1, 1, 0],
            [-1, 1, -1, 1, -1, 0, 1, 0, 0],
            [1, -1, 0, 0, 1, -1, 0, 0, -1],
            [0, -1, 0, 1, 0, 0, 0, -1, 1],
            [-1, 0, -1, 1, 0, 0, 0, 0, 1],
            [-1, 0, 1, 0, 1, 0, -1, 0, 0],
            [1, 1, -1, 0, 0, 0, 0, 0, -1],
            [0, 1, -1, 0, -1, 1, 0, 0, 0],
        ];
        let correct_moves: [usize; 12] = [7, 7, 0, 0, 0, 8, 2, 4, 1, 3, 5, 6];

        let it = cells_collections.iter().zip(correct_moves.iter());

        for (i, (cells, &correct_move)) in it.enumerate() {
            let game = init_game(cells, p1_mark, bot_mark, empty_mark, cells_to_win);

            match run_minimax_for_bot(game) {
                Some(best_move) => assert_eq!(best_move, correct_move, "collection {}", i + 1),
                _ => panic!("Bot::minimax (collection {i}) returned None"),
            }
        }
    }

    #[test]
    fn bot_prevent_early_win_move_3x3() {
        let (p1_mark, bot_mark, empty_mark) = (1, -1, 0);
        let cells_to_win = X33_CELLS_TO_WIN_MIN;

        let cells_collections: [[i8; 9]; 5] = [
            [1, 0, -1, 0, 0, 0, 0, 0, 1],
            [0, 0, 1, 0, 0, -1, 1, 0, 0],
            [0, -1, 0, 0, 0, 0, 1, 0, 1],
            [1, 0, 1, 0, 0, 0, 0, 0, -1],
            [1, 0, 1, -1, 0, 0, 0, 0, 0],
        ];
        let correct_moves: [usize; 5] = [4, 4, 7, 1, 1];

        let it = cells_collections.iter().zip(correct_moves.iter());

        for (i, (cells, &correct_move)) in it.enumerate() {
            let game = init_game(cells, p1_mark, bot_mark, empty_mark, cells_to_win);

            match run_minimax_for_bot(game) {
                Some(best_move) => assert_eq!(best_move, correct_move, "collection {}", i + 1),
                _ => panic!("Bot::minimax (collection {i}) returned None"),
            }
        }
    }

    #[test]
    fn bot_make_win_move_5x5() {
        let (p1_mark, bot_mark, empty_mark) = (1, -1, 0);
        let cells_to_win = X55_CELLS_TO_WIN_MIN;

        let cells_collections: [[i8; 25]; 8] = [
            [
                1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, -1, 0, -1, -1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            ],
            [
                0, 0, 0, 0, 0, 1, 0, -1, 0, 1, 0, 0, 0, 0, 0, 0, 0, -1, 0, 1, 0, 0, -1, 0, 0,
            ],
            [
                0, 0, 0, 1, 1, 0, -1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, -1, 0, 0, 0, 0, 0, -1,
            ],
            [
                1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, -1, -1, -1, 0, 1, 0, 0, 0, 0, 0,
            ],
            [
                0, -1, 0, -1, -1, 0, 1, -1, 1, 1, 1, -1, -1, -1, 1, 1, 0, 1, 0, 1, 0, 0, 0, 0, -1,
            ],
            [
                0, -1, 1, 0, 0, 0, 1, -1, 1, 0, 0, 1, -1, -1, 0, 1, -1, -1, 0, 0, 0, 0, 1, 0, 0,
            ],
            [
                0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 1, -1, 0, 0, 0, -1, 1, 0, 0, -1, 0, 0, 0,
            ],
            [
                1, 1, 0, 1, -1, 0, 0, 1, 0, 0, -1, -1, 0, -1, 0, 0, 0, 0, -1, 0, -1, 1, 0, 1, 1,
            ],
        ];
        let correct_win_moves: [usize; 8] = [12, 12, 12, 18, 2, 19, 9, 12];

        let it = cells_collections.iter().zip(correct_win_moves.iter());

        for (i, (cells, &correct_move)) in it.enumerate() {
            let game = init_game(cells, p1_mark, bot_mark, empty_mark, cells_to_win);

            match run_minimax_for_bot(game) {
                Some(best_move) => assert_eq!(best_move, correct_move, "collection {}", i + 1),
                _ => panic!("Bot::minimax (collection {i}) returned None"),
            }
        }
    }

    #[test]
    fn bot_prevent_win_move_5x5() {
        let (p1_mark, bot_mark, empty_mark) = (-1, 1, 0);
        let cells_to_win = X55_CELLS_TO_WIN_MIN;

        let cells_collections: [[i8; 25]; 8] = [
            [
                -1, 0, -1, -1, 1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            ],
            [
                0, 1, -1, -1, -1, 0, 0, 0, -1, -1, 0, 0, 0, 1, 0, 1, 1, 0, 0, -1, 0, 0, 0, 0, 1,
            ],
            [
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -1, 1, 0, 0, 0, 0, -1, 1, 0, 0, 0, 0, -1, 0,
            ],
            [
                0, 0, 0, 0, 0, 1, 0, 0, 1, 0, 1, -1, 1, -1, 0, 0, 0, -1, -1, -1, 0, 0, 0, 0, 0,
            ],
            [
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, -1, -1, -1, 0,
            ],
            [
                -1, 1, 0, 0, 0, -1, 0, 1, 0, 0, 0, 0, 0, 0, 0, -1, 0, 1, 0, -1, 0, 0, 0, 0, 0,
            ],
            [
                0, 1, 1, 0, 0, 0, 0, -1, 0, 0, 0, 0, 0, 0, 0, 0, 0, -1, 0, 0, 0, 0, -1, 0, 0,
            ],
            [
                0, 0, 0, 1, 1, 0, 1, -1, -1, 0, 0, -1, -1, 1, -1, 1, 1, 1, -1, -1, 0, 0, 0, 0, -1,
            ],
        ];
        let correct_moves: [usize; 8] = [1, 14, 5, 16, 24, 10, 12, 9];

        let it = cells_collections.iter().zip(correct_moves.iter());

        for (i, (cells, &correct_move)) in it.enumerate() {
            let game = init_game(cells, p1_mark, bot_mark, empty_mark, cells_to_win);

            match run_minimax_for_bot(game) {
                Some(best_move) => assert_eq!(best_move, correct_move, "collection {}", i + 1),
                _ => panic!("Bot::minimax (collection {i}) returned None"),
            }
        }
    }

    #[test]
    fn bot_game_final_move() {
        let (p1_mark, bot_mark, empty_mark) = (-1, 1, 0);
        let cells_to_win = X33_CELLS_TO_WIN_MIN;

        let cells_collections: [[i8; 9]; 3] = [
            [-1, 1, -1, -1, 1, 1, 1, -1, 0],
            [1, 1, 0, -1, -1, 1, 1, -1, -1],
            [1, -1, 0, 0, 1, -1, 0, 0, 0],
        ];
        let correct_next_move: [u8; 3] = [8, 2, 8];
        let correct_winner: [i8; 3] = [empty_mark, bot_mark, bot_mark];

        let it = cells_collections.iter().zip(0..correct_next_move.len());

        for (cells, i) in it {
            let game = init_game(cells, p1_mark, bot_mark, empty_mark, cells_to_win);

            let bot_move = Bot::next_move(game);

            assert_eq!(bot_move.next, correct_next_move[i]);
            assert_eq!(bot_move.winner, correct_winner[i]);
            assert_eq!(bot_move.next_is_valid, true);
            assert_eq!(bot_move.game_over, true);
        }
    }

    #[test]
    fn bot_game_final_move_markers_correct() {
        let (p1_mark, bot_mark, empty_mark) = (5, -3, 2);
        let cells_to_win = X33_CELLS_TO_WIN_MIN;

        let cells_collections: [[i8; 9]; 2] = [
            [5, 5, -3, 2, -3, 2, 2, 2, 2],
            [5, -3, -3, -3, -3, 5, 5, 5, 2],
        ];

        let correct_next_move: [u8; 2] = [6, 8];
        let correct_winner: [i8; 2] = [bot_mark, empty_mark];

        let it = cells_collections.iter().zip(0..correct_next_move.len());

        for (cells, i) in it {
            let game = init_game(cells, p1_mark, bot_mark, empty_mark, cells_to_win);

            let bot_move = Bot::next_move(game);

            // Check that player markers are the original and not the normalized
            assert_eq!(bot_move.next, correct_next_move[i]);
            assert_eq!(bot_move.winner, correct_winner[i]);
            assert_eq!(bot_move.next_is_valid, true);
            assert_eq!(bot_move.game_over, true);
        }
    }

    #[test]
    fn bot_not_game_final_move() {
        let (p1_mark, bot_mark, empty_mark) = (-1, 1, 0);
        let cells_to_win = X33_CELLS_TO_WIN_MIN;

        let cells_collections: [[i8; 9]; 3] = [
            [1, 0, -1, -1, 1, 0, 1, 0, -1],
            [1, -1, 1, 0, 1, 0, -1, 0, -1],
            [-1, 0, 0, 0, 0, 0, 1, 0, -1],
        ];
        let correct_next_move: [u8; 3] = [5, 7, 4];

        let it = cells_collections.iter().zip(0..correct_next_move.len());

        for (cells, i) in it {
            let game = init_game(cells, p1_mark, bot_mark, empty_mark, cells_to_win);

            let bot_move = Bot::next_move(game);

            assert_eq!(bot_move.next, correct_next_move[i]);
            assert_eq!(bot_move.winner, empty_mark);
            assert_eq!(bot_move.next_is_valid, true);
            assert_eq!(bot_move.game_over, false);
        }
    }

    #[test]
    fn bot_game_already_over() {
        let (p1_mark, bot_mark, empty_mark) = (-1, 1, 0);
        let cells_to_win = X55_CELLS_TO_WIN_MIN;

        let cells_collections: [[i8; 25]; 3] = [
            [
                0, 1, 1, 0, 0, 0, -1, -1, -1, -1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            ],
            [
                0, 0, 1, -1, -1, 0, 0, 1, -1, 0, 0, 0, 1, -1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0,
            ],
            [
                -1, 1, 1, -1, -1, -1, 1, -1, 1, 1, -1, 1, 1, -1, -1, 1, -1, 1, -1, 1, -1, 1, -1, 1,
                -1,
            ],
        ];
        let correct_winner: [i8; 3] = [p1_mark, bot_mark, empty_mark];

        let it = cells_collections.iter().zip(0..correct_winner.len());

        for (cells, i) in it {
            let game = init_game(cells, p1_mark, bot_mark, empty_mark, cells_to_win);

            let bot_move = Bot::next_move(game);

            assert_eq!(bot_move.winner, correct_winner[i]);
            assert_eq!(bot_move.game_over, true);
            assert_eq!(bot_move.next_is_valid, false);
        }
    }

    fn play_complete_game(
        game_size: usize,
        cells_to_win: u8,
        play_p1_first_move: bool,
        p1_first_move_idx: usize,
    ) {
        let empty_mark = 0;
        let (mut p1_mark, mut bot_mark) = (-1, 1);
        let mut cells = vec![0; game_size];

        let mut move_i = 0;

        if play_p1_first_move {
            cells[p1_first_move_idx] = p1_mark;
            move_i += 1;
        }

        loop {
            let board = Board {
                cells: cells.clone(),
                cells_to_win,
                p1_mark,
                bot_mark,
                empty_mark,
            };
            let game = match Game::new(board, Level::Normal) {
                Ok(game) => game,
                Err(error_kind) => panic!("Game::new() error {:?} in move {}", error_kind, move_i),
            };
            // Use following for debugging, add --nocapture after --
            println!("{}", &game);

            let res = Bot::next_move(game);

            move_i += 1;

            // Game status should be zero/tie after every move
            assert_eq!(res.winner, empty_mark, "move {}", move_i);

            match move_i.cmp(&game_size) {
                Ordering::Greater => {
                    // Game completed, no possible moves left
                    assert!(res.game_over, "move {}", move_i);
                    assert!(!res.next_is_valid, "move {}", move_i);
                    break;
                }
                Ordering::Equal => {
                    // Game completed after this last move
                    assert!(res.game_over, "move {}", move_i);
                }
                Ordering::Less => assert!(!res.game_over, "move {}", move_i),
            }

            assert!(res.next_is_valid, "move {}", move_i);
            let next_move = res.next as usize;
            assert!(next_move < game_size, "move {}", move_i);

            println!("next_move (idx): {next_move}\n");
            cells[next_move] = bot_mark;
            mem::swap(&mut bot_mark, &mut p1_mark);
        }
        // All cells should not be empty at this point
        let empties = cells.iter().filter(|&cell| *cell == empty_mark).count();
        assert_eq!(empties, 0);
    }

    // It's probably better to test with smaller `cells_to_win` values because those games are harder.
    // But too small values allows winning for one of the players which is not the goal of these test.
    // Games with maximal allowed `cells_to_win` should complete fast to a draw.
    // Overall, following test games should end up in a draw (make sure `cells_to_win` enables that).

    #[test]
    fn complete_game_play_3x3_3inarow_normal() {
        play_complete_game(9, 3, false, 0);
    }

    #[test]
    fn complete_game_play_4x4_4inarow_normal() {
        play_complete_game(16, 4, false, 0);
    }

    #[test]
    fn complete_game_play_5x5_4inarow_normal() {
        // Run separately with live board updates: cargo test --bin server complete_game_play_5x5_4inarow_normal -- --nocapture
        play_complete_game(25, 4, false, 0);
    }

    #[test]
    #[ignore]
    fn complete_game_play_5x5_4inarow_after_p1_center_move() {
        play_complete_game(25, 4, true, 12);
    }

    #[test]
    #[ignore]
    fn complete_game_play_5x5_4inarow_after_p1_lower_row_move() {
        // Run if there are doubts about the bot's answer to p1 opening move
        play_complete_game(25, 4, true, 21);
    }

    #[test]
    fn complete_game_play_6x6_5inarow_normal() {
        play_complete_game(36, 5, false, 0);
    }

    #[test]
    fn complete_game_play_7x7_5inarow_normal() {
        play_complete_game(49, 5, false, 0);
    }

    #[test]
    fn complete_game_play_8x8_5inarow_normal() {
        play_complete_game(64, 5, false, 0);
    }

    #[test]
    fn complete_game_play_9x9_5inarow_normal() {
        play_complete_game(81, 5, false, 0);
    }

    #[test]
    #[ignore]
    fn complete_game_play_10x10_5inarow_normal() {
        // How to run: cargo test --bin server complete_game_play_10x10_5inarow_normal -- --nocapture --ignored
        play_complete_game(100, 5, false, 0);
    }

    #[test]
    #[ignore]
    fn complete_game_play_11x11_5inarow_normal() {
        play_complete_game(11 * 11, 5, false, 0);
    }

    #[test]
    #[ignore]
    fn complete_game_play_12x12_5inarow_normal() {
        play_complete_game(12 * 12, 5, false, 0);
    }

    #[test]
    #[ignore]
    fn complete_game_play_13x13_5inarow_normal() {
        play_complete_game(13 * 13, 5, false, 0);
    }

    #[test]
    #[ignore]
    fn complete_game_play_14x14_5inarow_normal() {
        play_complete_game(14 * 14, 5, false, 0);
    }

    #[test]
    #[ignore]
    fn complete_game_play_15x15_5inarow_normal() {
        // Starting player should win this game
        play_complete_game(15 * 15, 5, false, 0);
    }
}
