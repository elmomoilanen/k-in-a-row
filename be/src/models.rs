use actix_web::web;
use serde::{Deserialize, Serialize};

/// Board representing current status of a k x k game.
///
/// Length of `cells` must be equal to the game board size k x k
/// such that first k cell values represent the first board row,
/// next k values second row and etc. Player marks are given
/// separately as `p1_mark` and `bot_mark` and must be different.
/// Empty mark `empty_mark` is used for cells without one of the
/// player marks. `Cells` must contain equal amount of player marks
/// or one more for `p1_mark` as the bot player plays next.
///
/// For example, in a 3x3 3-in-a-row game where the `p1_mark` has
/// made the first move of the game to the center cell
/// (2nd row and column) using -1 as the mark value, following `cells`
/// \[0, 0, 0, 0, -1, 0, 0, 0, 0\] would represent this situation
/// correctly with 0 representing the `empty_mark`.

#[derive(Deserialize, Clone, Debug)]
pub struct Board {
    pub cells: Vec<i8>,
    pub p1_mark: i8,
    pub bot_mark: i8,
    pub empty_mark: i8,
}

impl From<web::Json<Board>> for Board {
    fn from(board: web::Json<Board>) -> Self {
        Board {
            cells: board.cells.clone(),
            p1_mark: board.p1_mark,
            bot_mark: board.bot_mark,
            empty_mark: board.empty_mark,
        }
    }
}

/// URL query string parameter indicating difficulty of a game.
///
/// With a value `Normal` a game should always end to a draw if played
/// optimally by the player 1 (mark `p1_mark`). For value `Easy`, the
/// bot player (mark `bot_mark`) might make unoptimal decisions here
/// and there making it possible for player 1 to win.

#[derive(Deserialize, Clone, Debug)]
pub enum Level {
    Easy,
    Normal,
}

#[derive(Deserialize, Clone, Debug)]
pub struct LevelQuery {
    pub level: Level,
}

/// Bot player's next move and status of the game after this move.
///
/// `next` is the board index of the bot's next move. This index must
/// be interpreted such that values 0-(k-1) represent the first row of
/// the k x k board, k-(2k-1) the second row and etc. If `next_is_valid`
/// is true, this index is valid and can be played and updated to the
/// board. If not true, the index is just a garbage value (u8::MAX)
/// and cannot be played. Also, in this case, `game_over` is always true.
/// If `game_over` is true, `winner` is one of the player marks representing
/// the winner. If both `next_is_valid` and `game_over` are true, this
/// bot player's move `next` is the last one and ends the game.

#[derive(Serialize, Clone, Debug)]
pub struct BotMove {
    pub next: u8,
    pub next_is_valid: bool,
    pub game_over: bool,
    pub winner: i8,
}
