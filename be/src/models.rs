use actix_web::web;
use serde::{Deserialize, Serialize};

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

#[derive(Deserialize, Clone, Debug)]
pub enum Level {
    Easy,
    Normal,
}

#[derive(Deserialize, Clone, Debug)]
pub struct LevelQuery {
    pub level: Level,
}

#[derive(Serialize, Clone, Debug)]
pub struct BotMove {
    pub next: u8,
    pub next_is_valid: bool,
    pub game_over: bool,
    pub winner: i8,
}
