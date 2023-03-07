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

// Do not define boards over this size
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

// Must satisfy: to_win <= offset (for k x k board, offset is k)
const X33_CELLS_TO_WIN: u8 = 3; // draw at best for optimal play
const X44_CELLS_TO_WIN: u8 = 4; // draw
const X55_CELLS_TO_WIN: u8 = 4; // draw
const X66_CELLS_TO_WIN: u8 = 5; // draw
const X77_CELLS_TO_WIN: u8 = 5; // draw
const X88_CELLS_TO_WIN: u8 = 5; // draw
const X99_CELLS_TO_WIN: u8 = 6;
const X1010_CELLS_TO_WIN: u8 = 5;
const X1111_CELLS_TO_WIN: u8 = 5;
const X1212_CELLS_TO_WIN: u8 = 5;
const X1313_CELLS_TO_WIN: u8 = 6;
const X1414_CELLS_TO_WIN: u8 = 6;
const X1515_CELLS_TO_WIN: u8 = 5; // win

const X33_MAX_DEPTH: u8 = 9;
const X44_MAX_DEPTH: u8 = 8;
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
const X44_EASY_MAX_DEPTH: u8 = 1;
const X55_EASY_MAX_DEPTH: u8 = 2;
const X66_EASY_MAX_DEPTH: u8 = 1;
const X77_EASY_MAX_DEPTH: u8 = 2;
const X88_EASY_MAX_DEPTH: u8 = 1;
const X99_EASY_MAX_DEPTH: u8 = 2;
const X1010_EASY_MAX_DEPTH: u8 = 1;
const X1111_EASY_MAX_DEPTH: u8 = 2;
const X1212_EASY_MAX_DEPTH: u8 = 2;
const X1313_EASY_MAX_DEPTH: u8 = 1;
const X1414_EASY_MAX_DEPTH: u8 = 1;
const X1515_EASY_MAX_DEPTH: u8 = 1;

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
                return Ok(Self::init(board_size, level));
            }
        }
        Err(GameInitError::Size)
    }

    fn init(board_size: BoardSize, level: Level) -> Self {
        let offset = board_size as usize;

        let (to_win, max_depth) = match (board_size, level) {
            (BoardSize::X33, Level::Easy) => (X33_CELLS_TO_WIN, X33_EASY_MAX_DEPTH),
            (BoardSize::X33, Level::Normal) => (X33_CELLS_TO_WIN, X33_MAX_DEPTH),
            (BoardSize::X44, Level::Easy) => (X44_CELLS_TO_WIN, X44_EASY_MAX_DEPTH),
            (BoardSize::X44, Level::Normal) => (X44_CELLS_TO_WIN, X44_MAX_DEPTH),
            (BoardSize::X55, Level::Easy) => (X55_CELLS_TO_WIN, X55_EASY_MAX_DEPTH),
            (BoardSize::X55, Level::Normal) => (X55_CELLS_TO_WIN, X55_MAX_DEPTH),
            (BoardSize::X66, Level::Easy) => (X66_CELLS_TO_WIN, X66_EASY_MAX_DEPTH),
            (BoardSize::X66, Level::Normal) => (X66_CELLS_TO_WIN, X66_MAX_DEPTH),
            (BoardSize::X77, Level::Easy) => (X77_CELLS_TO_WIN, X77_EASY_MAX_DEPTH),
            (BoardSize::X77, Level::Normal) => (X77_CELLS_TO_WIN, X77_MAX_DEPTH),
            (BoardSize::X88, Level::Easy) => (X88_CELLS_TO_WIN, X88_EASY_MAX_DEPTH),
            (BoardSize::X88, Level::Normal) => (X88_CELLS_TO_WIN, X88_MAX_DEPTH),
            (BoardSize::X99, Level::Easy) => (X99_CELLS_TO_WIN, X99_EASY_MAX_DEPTH),
            (BoardSize::X99, Level::Normal) => (X99_CELLS_TO_WIN, X99_MAX_DEPTH),
            (BoardSize::X1010, Level::Easy) => (X1010_CELLS_TO_WIN, X1010_EASY_MAX_DEPTH),
            (BoardSize::X1010, Level::Normal) => (X1010_CELLS_TO_WIN, X1010_MAX_DEPTH),
            (BoardSize::X1111, Level::Easy) => (X1111_CELLS_TO_WIN, X1111_EASY_MAX_DEPTH),
            (BoardSize::X1111, Level::Normal) => (X1111_CELLS_TO_WIN, X1111_MAX_DEPTH),
            (BoardSize::X1212, Level::Easy) => (X1212_CELLS_TO_WIN, X1212_EASY_MAX_DEPTH),
            (BoardSize::X1212, Level::Normal) => (X1212_CELLS_TO_WIN, X1212_MAX_DEPTH),
            (BoardSize::X1313, Level::Easy) => (X1313_CELLS_TO_WIN, X1313_EASY_MAX_DEPTH),
            (BoardSize::X1313, Level::Normal) => (X1313_CELLS_TO_WIN, X1313_MAX_DEPTH),
            (BoardSize::X1414, Level::Easy) => (X1414_CELLS_TO_WIN, X1414_EASY_MAX_DEPTH),
            (BoardSize::X1414, Level::Normal) => (X1414_CELLS_TO_WIN, X1414_MAX_DEPTH),
            (BoardSize::X1515, Level::Easy) => (X1515_CELLS_TO_WIN, X1515_EASY_MAX_DEPTH),
            (BoardSize::X1515, Level::Normal) => (X1515_CELLS_TO_WIN, X1515_MAX_DEPTH),
        };

        BoardParams {
            size: board_size,
            offset,
            to_win: to_win as usize,
            max_depth: max_depth as usize,
        }
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
