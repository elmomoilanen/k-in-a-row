use actix_web::{guard, web, App, HttpServer};
use std::io;

#[path = "../bot.rs"]
mod bot;
#[path = "../game.rs"]
mod game;
#[path = "../handlers.rs"]
mod handlers;
#[path = "../models.rs"]
mod models;

use handlers::next_move;

const ADDR: &str = "0.0.0.0";
const PORT: u16 = 8080;

#[actix_web::main]
async fn main() -> io::Result<()> {
    HttpServer::new(|| {
        App::new().service(
            web::scope("/api").route(
                "/bot/next",
                web::post()
                    .guard(guard::Header("content-type", "application/json"))
                    .to(next_move),
            ),
        )
    })
    .bind((ADDR, PORT))?
    .run()
    .await
}
