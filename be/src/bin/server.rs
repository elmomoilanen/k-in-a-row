use actix_cors::Cors;
use actix_governor::{Governor, GovernorConfigBuilder};
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
#[path = "../guards.rs"]
mod guards;
#[path = "../handlers.rs"]
mod handlers;
#[path = "../models.rs"]
mod models;

use guards::RefererGuard;
use handlers::{hello, next_move};

const ALLOWED_DEV_CLIENT_URL: &str = "http://localhost:5173";
const ALLOWED_DEV_CLIENT_URL_2: &str = "http://127.0.0.1:5173";

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

    let client_url = env::var("CLIENT_URL").ok();

    let allowed_referers = if let Some(client_url) = &client_url {
        vec![client_url.clone()]
    } else {
        vec![
            ALLOWED_DEV_CLIENT_URL.to_string(),
            ALLOWED_DEV_CLIENT_URL_2.to_string(),
        ]
    };

    let governor_conf = GovernorConfigBuilder::default().finish().unwrap();

    HttpServer::new(move || {
        App::new()
            .wrap(
                if let Some(client_url) = &client_url {
                    Cors::default().allowed_origin(client_url)
                } else {
                    Cors::default()
                        .allowed_origin(ALLOWED_DEV_CLIENT_URL)
                        .allowed_origin(ALLOWED_DEV_CLIENT_URL_2)
                }
                .allowed_methods(vec!["GET", "POST"])
                .allowed_headers(vec![header::CONTENT_TYPE, header::ACCEPT])
                .max_age(3600),
            )
            .wrap(Governor::new(&governor_conf))
            .service(
                web::scope("/api")
                    .route(
                        "/bot/next",
                        web::post()
                            .guard(RefererGuard::new(allowed_referers.clone()))
                            .guard(guard::Header("content-type", "application/json"))
                            .to(next_move),
                    )
                    .route(
                        "/hello",
                        web::get()
                            .guard(RefererGuard::new(allowed_referers.clone()))
                            .to(hello),
                    ),
            )
    })
    .bind(address.clone())?
    .run()
    .await
}
