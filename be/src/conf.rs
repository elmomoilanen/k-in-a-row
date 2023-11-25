use crate::models::{Board, Level};

#[derive(Clone, Copy)]
pub enum BoardSize {
    X33 = 3,
    X44 = 4,
    X55 = 5,
    X66 = 6,
    X77 = 7,
    X88 = 8,
    X99 = 9,
    X1010 = 10,
    X1111 = 11,
    X1212 = 12,
    X1313 = 13,
    X1414 = 14,
    X1515 = 15,
}

// Do not define board sizes over this value
const MAX_BOARD_SIZE: u8 = u8::MAX;

const BOARD_SIZES: [(u8, BoardSize); 13] = [
    (9, BoardSize::X33),
    (16, BoardSize::X44),
    (25, BoardSize::X55),
    (36, BoardSize::X66),
    (49, BoardSize::X77),
    (64, BoardSize::X88),
    (81, BoardSize::X99),
    (100, BoardSize::X1010),
    (121, BoardSize::X1111),
    (144, BoardSize::X1212),
    (169, BoardSize::X1313),
    (196, BoardSize::X1414),
    (225, BoardSize::X1515),
];

pub const X33_CELLS_TO_WIN_MIN: u8 = 3;
pub const X44_CELLS_TO_WIN_MIN: u8 = 4;
pub const X55_CELLS_TO_WIN_MIN: u8 = 4;
pub const X66_CELLS_TO_WIN_MIN: u8 = 4;
pub const X77_CELLS_TO_WIN_MIN: u8 = 4;
pub const X88_CELLS_TO_WIN_MIN: u8 = 4;
pub const X99_CELLS_TO_WIN_MIN: u8 = 4;
pub const X1010_CELLS_TO_WIN_MIN: u8 = 5;
pub const X1111_CELLS_TO_WIN_MIN: u8 = 5;
pub const X1212_CELLS_TO_WIN_MIN: u8 = 5;
pub const X1313_CELLS_TO_WIN_MIN: u8 = 5;
pub const X1414_CELLS_TO_WIN_MIN: u8 = 5;
pub const X1515_CELLS_TO_WIN_MIN: u8 = 5;
// Max value must be less than or equal to offset (for a k x k board, offset is k)
pub const X33_CELLS_TO_WIN_MAX: u8 = 3;
pub const X44_CELLS_TO_WIN_MAX: u8 = 4;
pub const X55_CELLS_TO_WIN_MAX: u8 = 5;
pub const X66_CELLS_TO_WIN_MAX: u8 = 6;
pub const X77_CELLS_TO_WIN_MAX: u8 = 7;
pub const X88_CELLS_TO_WIN_MAX: u8 = 8;
pub const X99_CELLS_TO_WIN_MAX: u8 = 9;
pub const X1010_CELLS_TO_WIN_MAX: u8 = 10;
pub const X1111_CELLS_TO_WIN_MAX: u8 = 10;
pub const X1212_CELLS_TO_WIN_MAX: u8 = 10;
pub const X1313_CELLS_TO_WIN_MAX: u8 = 10;
pub const X1414_CELLS_TO_WIN_MAX: u8 = 10;
pub const X1515_CELLS_TO_WIN_MAX: u8 = 10;

const X33_MAX_DEPTH: u8 = 9;
const X44_MAX_DEPTH: u8 = 7;
const X55_MAX_DEPTH: u8 = 6;
const X66_MAX_DEPTH: u8 = 5;
const X77_MAX_DEPTH: u8 = 4;
const X88_MAX_DEPTH: u8 = 4;
const X99_MAX_DEPTH: u8 = 4;
const X1010_MAX_DEPTH: u8 = 3;
const X1111_MAX_DEPTH: u8 = 3;
const X1212_MAX_DEPTH: u8 = 3;
const X1313_MAX_DEPTH: u8 = 3;
const X1414_MAX_DEPTH: u8 = 3;
const X1515_MAX_DEPTH: u8 = 3;

const X33_EASY_MAX_DEPTH: u8 = 1;
const X44_EASY_MAX_DEPTH: u8 = 2;
const X55_EASY_MAX_DEPTH: u8 = 3;
const X66_EASY_MAX_DEPTH: u8 = 3;
const X77_EASY_MAX_DEPTH: u8 = 3;
const X88_EASY_MAX_DEPTH: u8 = 3;
const X99_EASY_MAX_DEPTH: u8 = 3;
const X1010_EASY_MAX_DEPTH: u8 = 2;
const X1111_EASY_MAX_DEPTH: u8 = 2;
const X1212_EASY_MAX_DEPTH: u8 = 2;
const X1313_EASY_MAX_DEPTH: u8 = 2;
const X1414_EASY_MAX_DEPTH: u8 = 2;
const X1515_EASY_MAX_DEPTH: u8 = 2;

pub struct BoardParams {
    pub size: BoardSize,
    pub offset: usize,
    pub to_win: usize,
    pub max_depth: usize,
}

#[derive(Clone, Copy, Debug)]
pub enum GameInitError {
    Size,
    Marks,
    Inconsistent,
    CellsToWin,
}

impl BoardParams {
    pub fn new(board: &Board, level: Level) -> Result<Self, GameInitError> {
        Self::board_inconsistent(board)?;

        if board.cells.len() > MAX_BOARD_SIZE as usize {
            return Err(GameInitError::Size);
        }

        let cells_total_count = board.cells.len() as u8;

        for &(cells_count, board_size) in BOARD_SIZES.iter() {
            if cells_total_count == cells_count {
                if let Ok(board_params) = Self::init(board_size, level, board.cells_to_win) {
                    return Ok(board_params);
                } else {
                    return Err(GameInitError::CellsToWin);
                }
            }
        }
        Err(GameInitError::Size)
    }

    fn init(board_size: BoardSize, level: Level, cells_to_win: u8) -> Result<Self, GameInitError> {
        let offset = board_size as usize;

        let (to_win, max_depth) = match (board_size, level, cells_to_win) {
            (BoardSize::X33, Level::Easy, (X33_CELLS_TO_WIN_MIN..=X33_CELLS_TO_WIN_MAX)) => {
                (cells_to_win, X33_EASY_MAX_DEPTH)
            }
            (BoardSize::X33, Level::Normal, (X33_CELLS_TO_WIN_MIN..=X33_CELLS_TO_WIN_MAX)) => {
                (cells_to_win, X33_MAX_DEPTH)
            }
            (BoardSize::X44, Level::Easy, (X44_CELLS_TO_WIN_MIN..=X44_CELLS_TO_WIN_MAX)) => {
                (cells_to_win, X44_EASY_MAX_DEPTH)
            }
            (BoardSize::X44, Level::Normal, (X44_CELLS_TO_WIN_MIN..=X44_CELLS_TO_WIN_MAX)) => {
                (cells_to_win, X44_MAX_DEPTH)
            }
            (BoardSize::X55, Level::Easy, (X55_CELLS_TO_WIN_MIN..=X55_CELLS_TO_WIN_MAX)) => {
                (cells_to_win, X55_EASY_MAX_DEPTH)
            }
            (BoardSize::X55, Level::Normal, (X55_CELLS_TO_WIN_MIN..=X55_CELLS_TO_WIN_MAX)) => {
                (cells_to_win, X55_MAX_DEPTH)
            }
            (BoardSize::X66, Level::Easy, (X66_CELLS_TO_WIN_MIN..=X66_CELLS_TO_WIN_MAX)) => {
                (cells_to_win, X66_EASY_MAX_DEPTH)
            }
            (BoardSize::X66, Level::Normal, (X66_CELLS_TO_WIN_MIN..=X66_CELLS_TO_WIN_MAX)) => {
                (cells_to_win, X66_MAX_DEPTH)
            }
            (BoardSize::X77, Level::Easy, (X77_CELLS_TO_WIN_MIN..=X77_CELLS_TO_WIN_MAX)) => {
                (cells_to_win, X77_EASY_MAX_DEPTH)
            }
            (BoardSize::X77, Level::Normal, (X77_CELLS_TO_WIN_MIN..=X77_CELLS_TO_WIN_MAX)) => {
                (cells_to_win, X77_MAX_DEPTH)
            }
            (BoardSize::X88, Level::Easy, (X88_CELLS_TO_WIN_MIN..=X88_CELLS_TO_WIN_MAX)) => {
                (cells_to_win, X88_EASY_MAX_DEPTH)
            }
            (BoardSize::X88, Level::Normal, (X88_CELLS_TO_WIN_MIN..=X88_CELLS_TO_WIN_MAX)) => {
                (cells_to_win, X88_MAX_DEPTH)
            }
            (BoardSize::X99, Level::Easy, (X99_CELLS_TO_WIN_MIN..=X99_CELLS_TO_WIN_MAX)) => {
                (cells_to_win, X99_EASY_MAX_DEPTH)
            }
            (BoardSize::X99, Level::Normal, (X99_CELLS_TO_WIN_MIN..=X99_CELLS_TO_WIN_MAX)) => {
                (cells_to_win, X99_MAX_DEPTH)
            }
            (BoardSize::X1010, Level::Easy, (X1010_CELLS_TO_WIN_MIN..=X1010_CELLS_TO_WIN_MAX)) => {
                (cells_to_win, X1010_EASY_MAX_DEPTH)
            }
            (
                BoardSize::X1010,
                Level::Normal,
                (X1010_CELLS_TO_WIN_MIN..=X1010_CELLS_TO_WIN_MAX),
            ) => (cells_to_win, X1010_MAX_DEPTH),
            (BoardSize::X1111, Level::Easy, (X1111_CELLS_TO_WIN_MIN..=X1111_CELLS_TO_WIN_MAX)) => {
                (cells_to_win, X1111_EASY_MAX_DEPTH)
            }
            (
                BoardSize::X1111,
                Level::Normal,
                (X1111_CELLS_TO_WIN_MIN..=X1111_CELLS_TO_WIN_MAX),
            ) => (cells_to_win, X1111_MAX_DEPTH),
            (BoardSize::X1212, Level::Easy, (X1212_CELLS_TO_WIN_MIN..=X1212_CELLS_TO_WIN_MAX)) => {
                (cells_to_win, X1212_EASY_MAX_DEPTH)
            }
            (
                BoardSize::X1212,
                Level::Normal,
                (X1212_CELLS_TO_WIN_MIN..=X1212_CELLS_TO_WIN_MAX),
            ) => (cells_to_win, X1212_MAX_DEPTH),
            (BoardSize::X1313, Level::Easy, (X1313_CELLS_TO_WIN_MIN..=X1313_CELLS_TO_WIN_MAX)) => {
                (cells_to_win, X1313_EASY_MAX_DEPTH)
            }
            (
                BoardSize::X1313,
                Level::Normal,
                (X1313_CELLS_TO_WIN_MIN..=X1313_CELLS_TO_WIN_MAX),
            ) => (cells_to_win, X1313_MAX_DEPTH),
            (BoardSize::X1414, Level::Easy, (X1414_CELLS_TO_WIN_MIN..=X1414_CELLS_TO_WIN_MAX)) => {
                (cells_to_win, X1414_EASY_MAX_DEPTH)
            }
            (
                BoardSize::X1414,
                Level::Normal,
                (X1414_CELLS_TO_WIN_MIN..=X1414_CELLS_TO_WIN_MAX),
            ) => (cells_to_win, X1414_MAX_DEPTH),
            (BoardSize::X1515, Level::Easy, (X1515_CELLS_TO_WIN_MIN..=X1515_CELLS_TO_WIN_MAX)) => {
                (cells_to_win, X1515_EASY_MAX_DEPTH)
            }
            (
                BoardSize::X1515,
                Level::Normal,
                (X1515_CELLS_TO_WIN_MIN..=X1515_CELLS_TO_WIN_MAX),
            ) => (cells_to_win, X1515_MAX_DEPTH),
            _ => return Err(GameInitError::CellsToWin),
        };

        Ok(BoardParams {
            size: board_size,
            offset,
            to_win: to_win as usize,
            max_depth: max_depth as usize,
        })
    }

    fn board_inconsistent(board: &Board) -> Result<(), GameInitError> {
        if board.p1_mark == board.bot_mark
            || board.p1_mark == board.empty_mark
            || board.bot_mark == board.empty_mark
        {
            return Err(GameInitError::Marks);
        }

        if board.cells.iter().any(|&cell| {
            cell != board.p1_mark && cell != board.bot_mark && cell != board.empty_mark
        }) {
            return Err(GameInitError::Marks);
        }

        let p1_marks = board
            .cells
            .iter()
            .filter(|&cell| *cell == board.p1_mark)
            .count();

        let bot_marks = board
            .cells
            .iter()
            .filter(|&cell| *cell == board.bot_mark)
            .count();

        if p1_marks > bot_marks + 1 || bot_marks > p1_marks {
            return Err(GameInitError::Inconsistent);
        }

        Ok(())
    }
}
