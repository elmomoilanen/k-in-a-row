use actix_cors::Cors;
use actix_web::{guard, http::header, web, App, HttpServer};
use std::{env, io};

#[path = "../bot.rs"]
mod bot;
#[path = "../conf.rs"]
mod conf;
#[path = "../first_move.rs"]
mod first_move;
#[path = "../game.rs"]
mod game;
#[path = "../handlers.rs"]
mod handlers;
#[path = "../models.rs"]
mod models;

use handlers::{hello, next_move};

#[actix_web::main]
async fn main() -> io::Result<()> {
    let port = match env::var("PORT") {
        Ok(port) => port,
        _ => String::from("8080"),
    };
    let addr = match env::var("ADDR") {
        Ok(addr) => addr,
        _ => String::from("0.0.0.0"),
    };
    let address = format!("{addr}:{port}");

    HttpServer::new(|| {
        App::new()
            .wrap(
                Cors::default()
                    .allowed_origin("http://localhost:5173")
                    .allowed_origin("http://127.0.0.1:5173")
                    .allowed_methods(vec!["GET", "POST"])
                    .allowed_headers(vec![header::CONTENT_TYPE, header::ACCEPT])
                    .max_age(3600),
            )
            .service(
                web::scope("/api")
                    .route(
                        "/bot/next",
                        web::post()
                            .guard(guard::Header("content-type", "application/json"))
                            .to(next_move),
                    )
                    .route("/hello", web::get().to(hello)),
            )
    })
    .bind(address.clone())?
    .run()
    .await
}
