use actix_web::{self, web, HttpResponse, Responder};

use crate::{
    bot::Bot,
    conf::GameInitError,
    game::Game,
    models::{Board, LevelQuery},
};

pub async fn hello() -> impl Responder {
    HttpResponse::Ok()
}

pub async fn next_move(
    game_board: web::Json<Board>,
    game_level: web::Query<LevelQuery>,
) -> impl Responder {
    let board = game_board.into_inner();
    let level = game_level.into_inner().level;

    let game = match Game::new(board, level) {
        Ok(game) => game,
        Err(GameInitError::Size) => {
            return HttpResponse::BadRequest().body("Board size is unaccepted.");
        }
        Err(GameInitError::Marks) => {
            return HttpResponse::BadRequest().body("Player markers in the board are unaccepted.");
        }
        Err(GameInitError::Inconsistent) => {
            return HttpResponse::BadRequest().body("Board state is inconsistent.");
        }
        Err(GameInitError::CellsToWin) => {
            return HttpResponse::BadRequest().body("Cells to win value is unaccepted.");
        }
    };

    let bot_next_move = Bot::next_move(game);

    HttpResponse::Ok().json(bot_next_move)
}
